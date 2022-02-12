use crate::ble_spec::{CLASSIC_CONTROL_CHARACTERISTIC_UUID, DIYMOTIONCONTROLLER_SERVICE_UUID};
use async_trait::async_trait;
use btleplug::api::{
    Central, CharPropFlags, Characteristic, Peripheral as _, ScanFilter, ValueNotification,
    WriteType,
};
use btleplug::platform::{Adapter, Peripheral};
use futures::stream::StreamExt;
use futures::Stream;
use std::pin::Pin;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time;
use tokio_stream::wrappers::IntervalStream;

#[async_trait]
pub trait Controller {
    // Connects the controller.
    async fn connect(&mut self);

    // Disconnectes the contorller
    async fn disconnect(&mut self);

    // Returns the update stream.
    async fn update_stream(&mut self) -> Pin<Box<dyn Stream<Item = ValueNotification>>>;

    // Writes to a particular characteristic.
    async fn write(&mut self, characteristic_uuid: &uuid::Uuid, value: &Vec<u8>);

    // This is meant to be actively checking that the connection is still active.
    async fn is_connected(&mut self) -> bool;
}

pub struct FakeController;

#[async_trait]
impl Controller for FakeController {
    async fn write(&mut self, characteristic_uuid: &uuid::Uuid, value: &Vec<u8>) {
        println!(
            "To characteristic {} sent value {:02X?}.",
            characteristic_uuid, value
        );
    }

    async fn is_connected(&mut self) -> bool {
        true
    }

    async fn update_stream(&mut self) -> Pin<Box<dyn Stream<Item = ValueNotification>>> {
        let interval = tokio::time::interval(Duration::from_secs(1));
        let interval_stream = IntervalStream::new(interval);
        Box::pin(interval_stream.map(move |_| {
            let ts = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            ValueNotification {
                value: ts.to_ne_bytes()[0..5].to_vec(),
                uuid: CLASSIC_CONTROL_CHARACTERISTIC_UUID,
            }
        }))
    }

    async fn connect(&mut self) {} // Do nothing.
    async fn disconnect(&mut self) {} // Do nothing.
}

pub struct BluetoothConnectedController {
    peripheral: Peripheral,
}

impl BluetoothConnectedController {
    fn is_subscribeable(characteristic: &Characteristic) -> bool {
        !(characteristic.properties & (CharPropFlags::READ | CharPropFlags::NOTIFY)).is_empty()
    }

    async fn wait_for_motion_controller(central: &Adapter) -> Peripheral {
        // start scanning for devices
        let scan_filter = ScanFilter {
            services: vec![DIYMOTIONCONTROLLER_SERVICE_UUID],
        };
        central.start_scan(scan_filter).await.unwrap();
        loop {
            match Self::find_motion_controller(central).await {
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

    pub async fn new(central: &Adapter) -> BluetoothConnectedController {
        let controller = Self::wait_for_motion_controller(&central).await;
        BluetoothConnectedController {
            peripheral: controller,
        }
    }
}

#[async_trait]
impl Controller for BluetoothConnectedController {
    async fn write(&mut self, characteristic_uuid: &uuid::Uuid, value: &Vec<u8>) {
        // find the correct characteristic
        match self
            .peripheral
            .characteristics()
            .into_iter()
            .find(|e| e.uuid == *characteristic_uuid)
        {
            Some(characteristic) => self
                .peripheral
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
    async fn is_connected(&mut self) -> bool {
        match self.peripheral.is_connected().await {
            Ok(is_connected) => is_connected,
            Err(_) => false,
        }
    }
    async fn update_stream(&mut self) -> Pin<Box<dyn Stream<Item = ValueNotification>>> {
        self.peripheral.notifications().await.unwrap()
    }
    async fn connect(&mut self) {
        self.peripheral.connect().await.unwrap();
        println!("We are connected :party:");
        for characteristic in self.peripheral.characteristics() {
            // Subscribe to all characteristics that are readable and notify.
            if Self::is_subscribeable(&characteristic) {
                self.peripheral.subscribe(&characteristic).await.unwrap();
            }
        }
    }
    async fn disconnect(&mut self) {
        for characteristic in self.peripheral.characteristics() {
            if Self::is_subscribeable(&characteristic) {
                match self.peripheral.unsubscribe(&characteristic).await {
                    Ok(()) => {}
                    Err(_) => {}
                }
            }
        }
        self.peripheral.disconnect().await.unwrap();
    }
}
