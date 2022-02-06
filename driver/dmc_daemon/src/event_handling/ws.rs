use crate::{ble_connection::Controller, state::ControllerState};
use dmc::ClientCommand;
use std::error::Error;
use tungstenite::Message;
use uuid::Uuid;

pub async fn on_ws_message<C: Controller>(
    controller_state: &mut ControllerState,
    controller_handle: &mut C,
    msg: Message,
) -> Result<(), Box<dyn Error>> {
    // TODO implement the commands
    if let Message::Binary(data) = msg {
        match serde_json::from_slice::<ClientCommand>(&data).unwrap() {
            ClientCommand::LedSet { r, g, b } => {
                let characteristic_uuid = Uuid::nil();
                let new_value: Vec<u8> = vec![r, g, b];
                // TODO call the write with the correct characteristic
                // controller_handle .write(characteristic_uuid, new_value).await;
            }
            ClientCommand::RumbleStop => {}
            ClientCommand::RumbleStart => {}
            ClientCommand::RumbleBurst { length: _ } => {}
        }
    }
    Ok(())
}
