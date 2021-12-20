use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use std::error::Error;
use std::time::Duration;
use tokio::time;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.event_receiver() to fetch a channel and
    // be notified of new devices
    println!("Scanning for devices...");
    time::sleep(Duration::from_secs(10)).await;

    // find the device we're interested in
    detect_devices(&central).await;

    Ok(())
}

async fn detect_devices(central: &Adapter) {
    for p in central.peripherals().await.unwrap() {
        let properties = p.properties().await.unwrap().unwrap();
        let address = properties.address;
        let maybe_name = properties.local_name;

        match maybe_name.to_owned() {
            Some(name) => println!("Found {} ({})", address, name),
            None => println!("Found {}", address),
        }


        if maybe_name == Some(String::from("MotionController")) {
            let _ = p.connect().await;

            match p.is_connected().await {
                Ok(true) => {
                    for c in p.characteristics().iter() {
                        println!(" with characteristic {}", c.uuid.to_string());
                    }
                    let _ = p.disconnect().await;
                },
                _ => (),
            }
        }
    }
}
