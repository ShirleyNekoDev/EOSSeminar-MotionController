use btleplug::api::Manager as _;
use btleplug::platform::Manager;
use dmc_daemon::ble_connection::Controller;
use futures::stream::StreamExt;
use futures::SinkExt;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{accept_async, tungstenite::Error as TError, WebSocketStream};
use tungstenite::Message;

use dmc::{ClientCommand, ClientUpdate};

use dmc_daemon::{
    ble_connection::BluetoothConnectedController, event_handling::*, state::ControllerState,
};

#[derive(Debug)]
enum WebSocketTaskError {
    UnexpectedError,
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

#[derive(Debug)]
enum BLETaskError {
    UnexpectedError,
}

async fn ble_task(
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
    command_tx: broadcast::Sender<ClientCommand>,
) -> Result<(), BLETaskError> {
    let mut command_rx = command_tx.subscribe();

    let mut controller_state = ControllerState::new();

    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = match manager.adapters().await {
        Ok(adapters) => adapters,
        Err(e) => panic!("Failed to get the first bluetooth adapter: {}", e),
    };
    let central = adapters.into_iter().nth(0).unwrap();

    let mut controller_handle = BluetoothConnectedController::new(&central).await;
    // let mut controller_handle = dmc_daemon::ble_connection::FakeController;

    let mut notification_stream = controller_handle.update_stream().await;

    if controller_handle.is_connected().await {
        if update_tx.receiver_count() > 0 {
            update_tx.send(vec![ClientUpdate::Connected]).unwrap();
        }
    }

    loop {
        tokio::select! {
            Ok(command) = command_rx.recv() => {
                match on_ws_message(&mut controller_state, &mut controller_handle, command).await {
                    Ok(_) => {},
                    Err(_) => return Err(BLETaskError::UnexpectedError),
                }
            },
            Some(data) = notification_stream.next() => {
                match on_ble_notification(&mut controller_state, data.uuid, data.value) {
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
                if !controller_handle.is_connected().await {
                    if update_tx.receiver_count() > 0 {
                        update_tx.send(vec![ClientUpdate::Disconnected]).unwrap();
                    }
                }
            }
        }
    }
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

    #[cfg(target_family = "windows")]
    let _ = tokio::spawn(async {
        // TODO: put ViGem Client here
    })
    .await?;

    let _ = ws_thread.await?;
    let _ = ble_thread.await?;
    Ok(())
}
