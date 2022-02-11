use btleplug::api::ValueNotification;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::error::Error;
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
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
    ws_stream: WebSocketStream<TcpStream>,
) -> Result<(), WebSocketTaskError> {
    let mut update_rx = update_tx.subscribe();
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
                        let command = serde_json::from_str::<ClientCommand>(msg.as_str()).unwrap();
                        command_tx.send(command).unwrap();
                    },
                    Ok(Message::Close(_)) => break,
                    Ok(any) => println!("Received unhandled message type from web socket: {:?}", any),
                    Err(TError::ConnectionClosed) => break,
                    Err(TError::Protocol(_)) => break,
                    Err(e) => panic!("Failed to receive something from websocket: {}", e),
                }
            },
        }
    }
    Ok(())
}

enum BLETaskError {
    UnexpectedError,
}

async fn ble_task(
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
    command_tx: broadcast::Sender<ClientCommand>,
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

    let mut command_rx = command_tx.subscribe();

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
                        if update_tx.receiver_count() > 0 {
                            println!("Emitting {} packed client updates to {} listeners", chain.len(), update_tx.receiver_count());
                            update_tx.send(chain).unwrap();
                        } else {
                            println!("Ignore {} packed client updates because there are no listeners", chain.len());
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

async fn init_ws(
    command_tx: broadcast::Sender<ClientCommand>,
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
) -> Result<(), WebSocketTaskError> {
    let addr = "127.0.0.1:9001";
    let server = TcpListener::bind(&addr)
        .await
        .expect("Cannot listen on port 9001.");
    println!("WebSocket server ready on ws://{}", addr);

    while let Ok((stream, _)) = server.accept().await {
        let peer = stream.peer_addr().unwrap();
        println!("We are connected to a new client with address {}", peer);
        match accept_async(stream).await {
            Ok(ws_stream) => {
                println!("WebSocket connected successfully");
                tokio::spawn(ws_task(command_tx.clone(), update_tx.clone(), ws_stream));
            }
            Err(e) => println!("Failed to connect WS: {}", e),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create communcation channels
    let (update_tx, _) = broadcast::channel::<Vec<ClientUpdate>>(32);
    let (command_tx, _) = broadcast::channel::<ClientCommand>(32);

    // start WS thread
    let ws_thread = tokio::spawn(init_ws(command_tx.clone(), update_tx.clone()));

    // start BLE thread
    let ble_thread = tokio::spawn(ble_task(update_tx.clone(), command_tx.clone()));

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
    
    let (_, _) = tokio::join!(ws_thread, ble_thread);
    Ok(())
}
