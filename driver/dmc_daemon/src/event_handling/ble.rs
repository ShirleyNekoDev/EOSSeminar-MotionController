use crate::{ble_spec::CLASSIC_CONTROL_CHARACTERISTIC_UUID, state::ControllerState};
use futures::stream::SplitSink;
use futures::SinkExt;
use std::error::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tungstenite::Message;
use uuid::Uuid;

pub async fn on_ble_notification(
    controller_state: &mut ControllerState,
    ws_sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    uuid: Uuid,
    value: Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    match uuid {
        CLASSIC_CONTROL_CHARACTERISTIC_UUID => {
            if let Some(chain) =
                crate::state::build_classic_control_updates(controller_state, &value)
            {
                ws_sender.send(Message::text(chain)).await?;
            }
            println!("new controller state: {:?}", controller_state);
        }
        _ => {
            println!(
                "received notification from unknown ble characteristic: {}",
                uuid
            )
        }
    }
    println!(
        "From characteristic {} received value {:02X?}.",
        uuid, value
    );
    Ok(())
}
