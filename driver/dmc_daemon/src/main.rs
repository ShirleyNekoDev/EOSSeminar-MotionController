use btleplug::api::ValueNotification;
use futures::stream::StreamExt;
use futures::Sink;
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::wrappers::IntervalStream;
use tokio_tungstenite::{accept_async, tungstenite::Error as TError, WebSocketStream};
use tungstenite::Message;

use dmc_daemon::ble_spec::*;
use dmc_daemon::event_handling::*;
use dmc_daemon::state::ControllerState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let manager = Manager::new().await.unwrap();

    // // get the first bluetooth adapter
    // let adapters = manager.adapters().await?;
    // let central = adapters.into_iter().nth(0).unwrap();

    // let controller_peripheral = wait_for_motion_controller(&central).await;
    // let _ = controller_peripheral.connect().await?;

    // println!("We are connected :party:");
    // println!("found characteristics:");

    let ws_stream = wait_for_client_connection().await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // for characteristic in controller_peripheral.characteristics().iter() {
    //     controller_peripheral.subscribe(characteristic).await?;
    // }

    // for characteristic in controller_peripheral.characteristics().iter() {
    //     controller_peripheral.unsubscribe(characteristic).await?;
    // }

    // let mut notification_stream = controller_peripheral.notifications().await?;

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
    work_loop(
        &mut controller_state,
        &mut notification_stream,
        &mut ws_receiver,
        &mut ws_sender,
    )
    .await?;
    Ok(())
}

async fn work_loop<
    N: StreamExt<Item = ValueNotification> + Unpin,
    R: StreamExt<Item = Result<Message, tungstenite::Error>> + Unpin,
    S: Sink<Message> + Unpin,
>(
    controller_state: &mut ControllerState,
    notification_stream: &mut N,
    ws_receiver: &mut R,
    ws_sender: &mut S,
) -> Result<(), S::Error> {
    loop {
        tokio::select! {
            Some(data) = notification_stream.next() => {
                on_ble_notification(controller_state, ws_sender, data.uuid, data.value).await?;
            },
            Some(result) = ws_receiver.next() => {
                match result {
                    Ok(msg) =>
                        on_ws_message(controller_state, msg, |characteristic_uuid, characteristic_value| {
                            println!("To characteristic {} sent value {:02X?}.", characteristic_uuid, characteristic_value);
                        }),
                    Err(TError::ConnectionClosed) => break,
                    Err(e) => panic!("Failed to receive message from ws: {}", e),
                }
            },
        }
    }
    Ok(())
}

async fn wait_for_client_connection() -> Result<WebSocketStream<TcpStream>, Box<dyn Error>> {
    let addr = "127.0.0.1:9001";
    let server = TcpListener::bind(&addr)
        .await
        .expect("Cannot listen on port 9001.");

    println!("Waiting for connection on addres {}", addr);
    let (stream, _) = server.accept().await?;
    let peer = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("We are connected to peer with address {}", peer);
    let ws_stream = accept_async(stream)
        .await
        .expect("Failed to accept websocket.");
    println!("WebSocket connection successful");
    return Ok(ws_stream);
}
