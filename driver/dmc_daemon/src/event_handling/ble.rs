use crate::{ble_spec::CLASSIC_CONTROL_CHARACTERISTIC_UUID, state::ControllerState};
use dmc::ClientUpdate;
use std::error::Error;
use uuid::Uuid;

pub fn on_ble_notification(
    controller_state: &mut ControllerState,
    uuid: Uuid,
    value: Vec<u8>,
) -> Result<Option<Vec<ClientUpdate>>, Box<dyn Error>> {
    match uuid {
        CLASSIC_CONTROL_CHARACTERISTIC_UUID => {
            return Ok(crate::state::build_classic_control_updates(
                controller_state,
                &value,
            ))
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
    Ok(None)
}
