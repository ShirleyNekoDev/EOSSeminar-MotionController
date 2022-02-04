use futures::SinkExt;
use futures::Sink;
use tungstenite::Message;
use crate::{
    state::ControllerState,
    ble_spec::CLASSIC_CONTROL_CHARACTERISTIC_UUID
};
use uuid::Uuid;

pub async fn on_ble_notification<S: Sink<Message> + Unpin>(
    controller_state: &mut ControllerState,
    ws_sender: &mut S,
    uuid: Uuid,
    value: Vec<u8>,
) -> Result<(), S::Error> {
    if uuid == Uuid::parse_str(CLASSIC_CONTROL_CHARACTERISTIC_UUID).unwrap() {
        if let Some(chain) =
            crate::state::build_classic_control_updates(controller_state, &value)
        {
            ws_sender.send(Message::binary(chain)).await?;
        }
        println!("new controller state: {:?}", controller_state);
    }
    println!(
        "From characteristic {} received value {:02X?}.",
        uuid, value
    );
    Ok(())
}

