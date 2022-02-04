use crate::ble_spec::DIYMOTIONCONTROLLER_SERVICE_UUID;
use async_trait::async_trait;
use btleplug::api::{Central, Characteristic, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Peripheral};
use futures::stream::StreamExt;
use std::error::Error;
use tokio::time::{self, Duration};

#[async_trait]
pub trait Controller {
    async fn write(&mut self, characteristic: uuid::Uuid, value: Vec<u8>);
    async fn is_connected(&mut self) -> bool;
}

pub struct FakeController;
pub struct BluetoothConnectedController {}

#[async_trait]
impl Controller for FakeController {
    async fn write(&mut self, characteristic: uuid::Uuid, value: Vec<u8>) {
        println!(
            "To characteristic {} sent value {:02X?}.",
            characteristic, value
        );
    }
    async fn is_connected(&mut self) -> bool {
        true
    }
}

async fn listen_for_updates(
    controller: &Peripheral,
    characteristic: &Characteristic,
) -> Result<(), Box<dyn Error>> {
    controller.subscribe(characteristic).await?;
    // let update_stream = controller.notifications().await?;
    let mut notification_stream = controller.notifications().await?.take(16);
    // Process while the BLE connection is not broken or stopped.
    while let Some(data) = notification_stream.next().await {
        println!("uuid: {:?}, value:{:?}", data.uuid, data.value);
    }
    controller.unsubscribe(characteristic).await?;
    Ok(())
}

async fn wait_for_motion_controller(central: &Adapter) -> Peripheral {
    // start scanning for devices
    let scan_filter = ScanFilter {
        services: vec![DIYMOTIONCONTROLLER_SERVICE_UUID],
    };
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
