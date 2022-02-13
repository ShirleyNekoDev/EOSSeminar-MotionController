use std::net::SocketAddr;

use dmc::{ClientCommand, ClientUpdate};
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::Error as TError, WebSocketStream};
use tungstenite::Message;

#[derive(Debug)]
pub enum WebSocketTaskError {
    UnexpectedError,
}

pub async fn start_websocket_server(
    command_tx: broadcast::Sender<ClientCommand>,
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
    ws_address: SocketAddr,
) -> Result<(), WebSocketTaskError> {
    let server = TcpListener::bind(ws_address)
        .await
        .expect(format!("WebSocket could not be bound {}", ws_address).as_str());
    println!("WebSocket server ready on ws://{}", ws_address);

    while let Ok((stream, _)) = server.accept().await {
        let peer = stream.peer_addr().unwrap();
        println!("We are connected to a new client with address {}", peer);
        match accept_async(stream).await {
            Ok(ws_stream) => {
                println!("WebSocket connected successfully");
                tokio::spawn(handle_websocket_client(
                    command_tx.clone(),
                    update_tx.clone(),
                    ws_stream,
                ));
            }
            Err(e) => println!("Failed to connect WS: {}", e),
        }
    }

    Ok(())
}

async fn handle_websocket_client(
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
