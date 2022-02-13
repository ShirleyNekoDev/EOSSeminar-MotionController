use std::f32::consts::PI;
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use futures::{stream::select_all, Stream};
use futures::stream::StreamExt;
use tokio_stream::wrappers::IntervalStream;
use dmc::{ClientCommand, ClientUpdate};
use tokio::sync::broadcast;

use crate::state::ControllerState;
use crate::utils::SenderUtils;

pub async fn register_controller_mock(
    _command_tx: broadcast::Sender<ClientCommand>,
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
) {
    loop {
         select_all([
            fiddle_buttons(Duration::from_millis(800)),
            fiddle_joystick(Duration::from_millis(50)),
            update_battery(Duration::from_millis(1500)),
        ]).for_each(|data| async {
            update_tx.broadcast_updates(data).unwrap();
        }).await;
    }
}

fn interval_stream(period: Duration) -> IntervalStream {
    IntervalStream::new(tokio::time::interval(period))
}

fn fiddle_buttons (
    period: Duration
) -> Pin<Box<dyn Stream<Item = Vec<ClientUpdate>> + Send>> {
    let mut button_byte = 0_u8;
    let mut controller_state = ControllerState::new();
    Box::pin(
        interval_stream(period)
        .map(move |_| {
            button_byte += 1;
            if button_byte > 0b111 {
                button_byte = 0;
            }
            let button_a = (button_byte & 1u8 << 0) == 0;
            let button_b = (button_byte & 1u8 << 1) == 0;
            let button_menu = (button_byte & 1u8 << 2) == 0;

            controller_state.update_buttons(
                button_a.into(),
                button_b.into(),
                button_menu.into()
            )
        })
    )
}
fn fiddle_joystick(
    period: Duration
) -> Pin<Box<dyn Stream<Item = Vec<ClientUpdate>> + Send>> {
    let mut controller_state = ControllerState::new();
    Box::pin(
        interval_stream(period)
        .map(move |_| {
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() % 3600000;
            let angle = ts as f32 / 1000.0 * (PI / 10.0);
            vec![controller_state.dbg_set_joystick(angle, 1.0)]
        })
    )
}
fn update_battery(
    period: Duration
) -> Pin<Box<dyn Stream<Item = Vec<ClientUpdate>> + Send>> {
    let mut status = 255_u8;
    Box::pin(
        interval_stream(period)
        .map(move |_| {
            status = status.wrapping_sub(8);
            vec![ClientUpdate::BatteryStatusChanged{charge: status}]
        })
    )
}
