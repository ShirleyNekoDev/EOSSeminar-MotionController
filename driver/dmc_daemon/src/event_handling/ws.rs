use crate::{ble_connection::Controller, state::ControllerState};
use dmc::ClientCommand;
use tungstenite::Message;

pub fn on_ws_message<C: Controller>(
    controller_state: &mut ControllerState,
    controller_handle: &mut C,
    msg: Message,
) {
    if let Message::Binary(data) = msg {
        match bincode::deserialize::<ClientCommand>(&data).unwrap() {
            ClientCommand::LedSet { r: _, g: _, b: _ } => {}
            ClientCommand::RumbleStop => {}
            ClientCommand::RumbleStart => {}
            ClientCommand::RumbleBurst { length: _ } => {}
        }
    }
}
