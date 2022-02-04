use tungstenite::Message;
use uuid::Uuid;
use dmc::ClientCommand;
use crate::state::ControllerState;

pub fn on_ws_message(
    controller_state: &mut ControllerState,
    msg: Message,
    _controller_write_method: fn(Uuid, Vec<u8>),
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

