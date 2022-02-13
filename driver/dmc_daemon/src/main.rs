use dmc_daemon::controller_mock::register_controller_mock;
use std::error::Error;
use std::net::SocketAddrV4;
use tokio::sync::broadcast;

use dmc::{ClientCommand, ClientUpdate};
use dmc_daemon::event_handling::ble;
use dmc_daemon::event_handling::ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create communcation channels
    let (update_tx, _) = broadcast::channel::<Vec<ClientUpdate>>(32);
    let (command_tx, _) = broadcast::channel::<ClientCommand>(32);

    // start WS thread
    let ws_thread = tokio::spawn(ws::start_websocket_server(
        command_tx.clone(),
        update_tx.clone(),
        std::net::SocketAddr::V4(SocketAddrV4::new("127.0.0.1".parse().unwrap(), 9001)),
    ));

    // start BLE thread
    let ble_thread = tokio::spawn(ble::start_bluetooth_device_handler(
        command_tx.clone(),
        update_tx.clone(),
    ));

    // let mock_controller = tokio::spawn(register_controller_mock(
    //     command_tx.clone(),
    //     update_tx.clone(),
    // ));

    #[cfg(target_family = "windows")]
    let _ = tokio::spawn(async {
        // TODO: put ViGem Client here
    })
    .await?;

    let _ = ws_thread.await?;
    let _ = ble_thread.await?;
    // let _ = mock_controller.await?;
    Ok(())
}
