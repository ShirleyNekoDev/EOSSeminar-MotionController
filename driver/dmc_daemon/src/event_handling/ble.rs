use crate::{
    ble_spec::{
        CLASSIC_CONTROL_CHARACTERISTIC_UUID, DIYMOTIONCONTROLLER_SERVICE_UUID,
        FEEDBACK_CHARACTERISTIC_UUID,
    },
    state::ControllerState,
    utils::SenderUtils,
};
use btleplug::api::{
    Central, CharPropFlags, Characteristic, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::Manager;
use btleplug::{api::Manager as _, platform::Adapter, platform::Peripheral};
use dmc::{ClientCommand, ClientUpdate};
use futures::stream::StreamExt;
use std::error::Error;
use tokio::time::{sleep, Duration};
use tokio::{
    sync::broadcast::{self},
    time,
};
use uuid::Uuid;

#[derive(Debug)]
pub enum BLETaskError {
    UnexpectedError,
}

async fn init_bluetooth_adapter() -> Adapter {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = match manager.adapters().await {
        Ok(adapters) => adapters,
        Err(e) => panic!("Failed to get the first bluetooth adapter: {}", e),
    };
    let central = adapters.into_iter().nth(0).unwrap();
    central
}

pub async fn start_bluetooth_device_handler(
    command_tx: broadcast::Sender<ClientCommand>,
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
) -> Result<(), BLETaskError> {
    let adapter = init_bluetooth_adapter().await;
    handle_ble_device(command_tx, update_tx, adapter).await
}

async fn handle_ble_device(
    command_tx: broadcast::Sender<ClientCommand>,
    update_tx: broadcast::Sender<Vec<ClientUpdate>>,
    adapter: Adapter,
) -> Result<(), BLETaskError> {
    let mut controller_state = ControllerState::new();

    let peripheral = wait_for_motion_controller(&adapter).await;

    let mut command_rx = command_tx.subscribe();

    let mut notification_stream = peripheral.notifications().await.unwrap();

    connect(&peripheral).await;
    if is_connected(&peripheral).await {
        update_tx.broadcast_update(ClientUpdate::Connected).unwrap();
    }

    loop {
        tokio::select! {
            Ok(command) = command_rx.recv() => {
                match notify_ble_for_client_command(&mut controller_state, &peripheral, command).await {
                    Ok(_) => {},
                    Err(_) => break,
                }
            },
            Some(data) = notification_stream.next() => {
                println!("Received notification from characteristic with UUID {}", data.uuid);
                match pack_client_updates_for_ble_notification(&mut controller_state, data.uuid, data.value) {
                    Ok(Some(chain)) => {
                        println!("Emitting {} packed client updates to {} listeners", chain.len(), update_tx.receiver_count());
                        update_tx.broadcast_updates(chain).unwrap();
                    },
                    Ok(None) => (),
                    Err(e) => {
                        println!("Error while receiving BLE notification: {}", e);
                        break
                    },
                }
            },
            _ = sleep(Duration::from_secs(10)) => {
                if !is_connected(&peripheral).await {
                    update_tx.broadcast_update(ClientUpdate::Disconnected).unwrap();
                }
            }
        }
    }
    disconnect(&peripheral).await;
    if !is_connected(&peripheral).await {
        update_tx
            .broadcast_update(ClientUpdate::Disconnected)
            .unwrap();
    }
    Err(BLETaskError::UnexpectedError)
}

fn pack_client_updates_for_ble_notification(
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

async fn notify_ble_for_client_command(
    _controller_state: &mut ControllerState,
    peripheral: &Peripheral,
    command: ClientCommand,
) -> Result<(), Box<dyn Error>> {
    // TODO update the controller state somehow (as of now it does not hold Feedback information
    // TODO implement the commands
    match command {
        ClientCommand::LedSet { r, g, b } => {
            let new_value: Vec<u8> = vec![r, g, b];
            write(peripheral, &FEEDBACK_CHARACTERISTIC_UUID, &new_value).await;
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

async fn wait_for_motion_controller(central: &Adapter) -> Peripheral {
    // start scanning for devices
    let scan_filter = ScanFilter {
        services: vec![DIYMOTIONCONTROLLER_SERVICE_UUID],
    };
    println!("Starting scan for controller...");
    central.start_scan(scan_filter).await.unwrap();
    loop {
        match find_motion_controller(central).await {
            Some(peripheral) => {
                return peripheral;
            }
            None => {
                time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
}
async fn find_motion_controller(central: &Adapter) -> Option<Peripheral> {
    for peripheral in central.peripherals().await.unwrap() {
        let properties = peripheral.properties().await.unwrap().unwrap();
        let maybe_name = properties.local_name;
        if maybe_name == Some(String::from("DIYMotionController")) {
            println!("Found device at {:?}", properties.address);
            return Some(peripheral);
        }
    }
    return None;
}

async fn write(peripheral: &Peripheral, characteristic_uuid: &uuid::Uuid, value: &Vec<u8>) {
    // find the correct characteristic
    match peripheral
        .characteristics()
        .into_iter()
        .find(|e| e.uuid == *characteristic_uuid)
    {
        Some(characteristic) => peripheral
            .write(
                &characteristic,
                value.as_slice(),
                WriteType::WithoutResponse,
            )
            .await
            .unwrap(),
        None => println!(
            "Did not find characteristic with uuid {}",
            characteristic_uuid
        ),
    }
}

async fn connect(peripheral: &Peripheral) {
    peripheral.connect().await.unwrap();
    println!("We are connected to the BLE peripheral");
    peripheral.discover_services().await.unwrap();
    println!("We have discovered services");
    for characteristic in peripheral.characteristics() {
        // Subscribe to all characteristics that are readable and notify.
        println!(
            "Looking at characteristic with UUID {}",
            characteristic.uuid
        );
        if is_subscribeable(&characteristic) {
            println!(
                "Subscribing characteristic with UUID {}",
                characteristic.uuid
            );
            peripheral.subscribe(&characteristic).await.unwrap();
        }
    }
}
async fn disconnect(peripheral: &Peripheral) {
    for characteristic in peripheral.characteristics() {
        if is_subscribeable(&characteristic) {
            match peripheral.unsubscribe(&characteristic).await {
                Ok(()) => {}
                Err(_) => {}
            }
        }
    }
    peripheral.disconnect().await.unwrap();
}

fn is_subscribeable(characteristic: &Characteristic) -> bool {
    !(characteristic.properties & (CharPropFlags::READ | CharPropFlags::NOTIFY)).is_empty()
        && [CLASSIC_CONTROL_CHARACTERISTIC_UUID, FEEDBACK_CHARACTERISTIC_UUID].contains(&characteristic.uuid)
}

async fn is_connected(peripheral: &Peripheral) -> bool {
    match peripheral.is_connected().await {
        Ok(is_connected) => is_connected,
        Err(_) => false,
    }
}
