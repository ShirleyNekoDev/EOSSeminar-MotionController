use crate::{ble_connection::Controller, state::ControllerState};
use dmc::ClientCommand;
use std::error::Error;
use uuid::Uuid;

pub async fn on_ws_message<C: Controller>(
    controller_state: &mut ControllerState,
    controller_handle: &mut C,
    command: ClientCommand,
) -> Result<(), Box<dyn Error>> {
    // TODO implement the commands
    match command {
        ClientCommand::LedSet { r, g, b } => {
            let characteristic_uuid = Uuid::nil();
            let new_value: Vec<u8> = vec![r, g, b];
            // TODO call the write with the correct characteristic
            // controller_handle .write(characteristic_uuid, new_value) .await;
            println!(
                "Setting RGB led to r={} g={} b={}",
                new_value[0], new_value[1], new_value[2]
            );
        }
        ClientCommand::RumbleStop => {}
        ClientCommand::RumbleStart => {}
        ClientCommand::RumbleBurst { length: _ } => {}
    }
    Ok(())
}
