use btleplug::api::ValueNotification;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::error::Error;
use std::sync::{
    atomic::{AtomicUsize, Ordering::Relaxed},
    Arc,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::time::sleep;
use tokio_stream::wrappers::IntervalStream;
use tokio_tungstenite::{accept_async, tungstenite::Error as TError, WebSocketStream};
use tungstenite::Message;

use dmc::{ClientCommand, ClientUpdate};

use dmc_daemon::{
    ble_connection::FakeController, ble_spec::*, event_handling::*, state::ControllerState,
};

enum WebSocketTaskError {
    UnexpectedError,
}

async fn ws_task(
    command_tx: broadcast::Sender<ClientCommand>,
    mut update_rx: broadcast::Receiver<Vec<ClientUpdate>>,
    clients_connected: Arc<AtomicUsize>,
) -> Result<(), WebSocketTaskError> {
    let addr = "127.0.0.1:9001";
    let server = TcpListener::bind(&addr)
        .await
        .expect("Cannot listen on port 9001.");
    println!("WebSocket server ready on ws://{}", addr);

    loop {
        let ws_stream = match wait_for_client_connection(&server).await {
            Ok(ws_stream) => ws_stream,
            Err(_) => return Err(WebSocketTaskError::UnexpectedError),
        };
        clients_connected.fetch_add(1, Relaxed);
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        loop {
            tokio::select! {
                result = update_rx.recv() => {
                    match result {
                        Ok(msgs) => {
                            let update_pack = serde_json::to_string(&msgs).unwrap();
                            match ws_sender.send(Message::text(update_pack)).await {
                                Ok(()) => (),
                                Err(_) => return Err(WebSocketTaskError::UnexpectedError),
                            };
                        },
                        Err(_) => {},
                    }
                },
                Some(result) = ws_receiver.next() => {
                    match result {
                        Ok(Message::Text(msg)) => {
                            // on_ws_message(controller_state, controller_handle, msg).await?,
                            let command = serde_json::from_str::<ClientCommand>(msg.as_str()).unwrap();
                            command_tx.send(command).unwrap();
                        },
                        Ok(_) => println!("Received NOT text message from web socket."),
                        Err(TError::ConnectionClosed) => break,
                        Err(TError::Protocol(_)) => break,
                        Err(e) => panic!("Failed to receive something from websocket: {}", e),
                    }
                },
            }
        }
        clients_connected.fetch_sub(1, Relaxed);
    }
}

enum BLETaskError {
    UnexpectedError,
}

async fn ble_task(
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
    mut command_rx: broadcast::Receiver<ClientCommand>,
    clients_connected: Arc<AtomicUsize>,
) -> Result<(), BLETaskError> {
    let interval = tokio::time::interval(Duration::from_secs(1));
    let interval_stream = IntervalStream::new(interval);
    let mut notification_stream = interval_stream.map(move |_| {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        ValueNotification {
            value: ts.to_ne_bytes()[0..5].to_vec(),
            uuid: CLASSIC_CONTROL_CHARACTERISTIC_UUID,
        }
    });

    let mut controller_state = ControllerState::new();

    let mut controller_handle = FakeController;

    loop {
        tokio::select! {
            Ok(command) = command_rx.recv() => {
                match on_ws_message(&mut controller_state, &mut controller_handle, command).await {
                    Ok(_) => {},
                    Err(_) => return Err(BLETaskError::UnexpectedError),
                }
            },
            Some(data) = notification_stream.next() => {
                match on_ble_notification(&mut controller_state, data.uuid, data.value).await {
                    Ok(Some(chain)) => {
                        if clients_connected.load(Relaxed) > 0 {
                            println!("Emitting {} client updates into channel...", chain.len());
                            update_tx.send(chain).unwrap();
                        }
                    },
                    Ok(None) => (),
                    Err(_) => return Err(BLETaskError::UnexpectedError),
                }
            },
            _ = sleep(Duration::from_secs(10)) => {
                // TODO check whether BLE connection is still up
            }

        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create communcation channels
    let (update_tx, update_rx) = broadcast::channel::<Vec<ClientUpdate>>(32);
    let (command_tx, command_rx) = broadcast::channel::<ClientCommand>(32);

    // create shared variables
    let clients_connected = Arc::new(AtomicUsize::new(0));

    // start WS thread
    let server_thread = tokio::spawn(ws_task(
        command_tx.clone(),
        update_rx,
        clients_connected.clone(),
    ));

    // start BLE thread
    let ble_thread = tokio::spawn(ble_task(
        update_tx.clone(),
        command_rx,
        clients_connected.clone(),
    ));

    // let manager = Manager::new().await.unwrap();

    // // get the first bluetooth adapter
    // let adapters = manager.adapters().await?;
    // let central = adapters.into_iter().nth(0).unwrap();

    // let controller_peripheral = wait_for_motion_controller(&central).await;
    // let _ = controller_peripheral.connect().await?;

    // println!("We are connected :party:");
    // println!("found characteristics:");

    // for characteristic in controller_peripheral.characteristics().iter() {
    //     controller_peripheral.subscribe(characteristic).await?;
    // }

    // for characteristic in controller_peripheral.characteristics().iter() {
    //     controller_peripheral.unsubscribe(characteristic).await?;
    // }

    // let mut notification_stream = controller_peripheral.notifications().await?;
    let _ = server_thread.await?;
    let _ = ble_thread.await?;
    Ok(())
}

async fn wait_for_client_connection(
    server: &TcpListener,
) -> Result<WebSocketStream<TcpStream>, Box<dyn Error>> {
    println!("Waiting for connection...");
    let (stream, _) = server.accept().await?;
    let peer = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("We are connected to a new peer with address {}", peer);
    let ws_stream = accept_async(stream)
        .await
        .expect("Failed to accept websocket.");
    println!("WebSocket connection successful");
    return Ok(ws_stream);
}
