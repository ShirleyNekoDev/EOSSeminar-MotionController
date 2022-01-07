use btleplug::api::{Central, Manager as _, Peripheral as _, Characteristic, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::option::Option;
use std::time::Duration;
use tokio::time;
use futures::stream::StreamExt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    let controller_peripheral = wait_for_motion_controller(&central).await;
    let _ = controller_peripheral.connect().await?;

    println!("We are connected :party:");
    println!("found characteristics:");

    for characteristic in controller_peripheral.characteristics().iter() {
        println!(" - {}", characteristic.uuid.to_string());
        listen_for_updates(&controller_peripheral, characteristic).await?;
    }

    Ok(())
}

async fn listen_for_updates(controller: &Peripheral, characteristic: &Characteristic) -> Result<(), Box<dyn Error>> {
    controller.subscribe(characteristic).await?;
    // let update_stream = controller.notifications().await?;
    let mut notification_stream = controller.notifications().await?.take(1);
    // Process while the BLE connection is not broken or stopped.
    while let Some(data) = notification_stream.next().await {
        println!(
            "uuid: {:?}, value:{:?}",
            data.uuid, data.value
        );
    }
    Ok(())
}

async fn wait_for_motion_controller(central: &Adapter) -> Peripheral {
    // start scanning for devices
    central.start_scan(ScanFilter::default()).await.unwrap();
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

// async fn detect_devices(central: &Adapter) {
//     for p in central.peripherals().await.unwrap() {
//         let properties = p.properties().await.unwrap().unwrap();
//         let address = properties.address;
//         let maybe_name = properties.local_name;

//         match maybe_name.to_owned() {
//             Some(name) => println!("Found {} ({})", address, name),
//             None => println!("Found {}", address),
//         }


//         if maybe_name == Some(String::from("DIYMotionController")) {
//             let _ = p.connect().await;

//             match p.is_connected().await {
//                 Ok(true) => {
//                     for c in p.characteristics().iter() {
//                         println!(" with characteristic {}", c.uuid.to_string());
//                     }
//                     let _ = p.disconnect().await;
//                 },
//                 _ => (),
//             }
//         }
//     }
// }
