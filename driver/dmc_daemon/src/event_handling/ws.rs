use crate::{
    ble_connection::Controller, ble_spec::FEEDBACK_CHARACTERISTIC_UUID, state::ControllerState,
};
use dmc::ClientCommand;
use std::error::Error;

pub async fn on_ws_message<C: Controller>(
    _controller_state: &mut ControllerState,
    controller_handle: &mut C,
    command: ClientCommand,
) -> Result<(), Box<dyn Error>> {
    // TODO update the controller state somehow (as of now it does not hold Feedback information
    // TODO implement the commands
    match command {
        ClientCommand::LedSet { r, g, b } => {
            let new_value: Vec<u8> = vec![r, g, b];
            controller_handle
                .write(&FEEDBACK_CHARACTERISTIC_UUID, &new_value)
                .await;
            println!(
                "Setting RGB led to r={} g={} b={}",
                new_value[0], new_value[1], new_value[2]
            );
        }
        ClientCommand::RumbleStop => {
            println!("The RumbleStop command ist not implement yet.");
        }
        ClientCommand::RumbleStart => {
            println!("The RumbleStart command ist not implement yet.");
        }
        ClientCommand::RumbleBurst { length } => {
            println!(
                "The RumbleBurst command ist not implement yet. (tried to burst with length {})",
                length
            );
        }
    }
    Ok(())
}
