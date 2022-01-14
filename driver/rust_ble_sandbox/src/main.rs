use btleplug::api::{Central, Manager as _, Peripheral as _, Characteristic, ScanFilter, ValueNotification};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio_stream::wrappers::IntervalStream;
use std::error::Error;
use std::option::Option;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;
use futures::stream::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream, tungstenite::Error as TError};
use tungstenite::Message;
use serde::{Serialize, Deserialize};


const DIYMOTIONCONTROLLER_SERVICE_UUID: &str = "328c9225-877f-4189-89a8-b50bb21b02ae";

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum ClientCommand {
    LedSet { r: u8, g: u8, b: u8 },
    RumbleStart,
    RumbleStop,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum ClientUpdate {

}

fn on_ws_message(msg: Message, _controller_write_method: fn(Uuid, Vec<u8>)) {
    if let Message::Binary(data) = msg {
        match bincode::deserialize::<ClientCommand>(&data).unwrap() {
            ClientCommand::LedSet { r: _, g: _, b: _ } => {

            },
            ClientCommand::RumbleStop => {

            },
            ClientCommand::RumbleStart => {

            },
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let manager = Manager::new().await.unwrap();

    // // get the first bluetooth adapter
    // let adapters = manager.adapters().await?;
    // let central = adapters.into_iter().nth(0).unwrap();

    // let controller_peripheral = wait_for_motion_controller(&central).await;
    // let _ = controller_peripheral.connect().await?;

    // println!("We are connected :party:");
    // println!("found characteristics:");

    let ws_stream = wait_for_client_connection().await?;
    let (mut _ws_sender, mut ws_receiver) = ws_stream.split();

    // for characteristic in controller_peripheral.characteristics().iter() {
    //     controller_peripheral.subscribe(characteristic).await?;
    // }

    // for characteristic in controller_peripheral.characteristics().iter() {
    //     controller_peripheral.unsubscribe(characteristic).await?;
    // }

    // let mut notification_stream = controller_peripheral.notifications().await?;

    let interval = tokio::time::interval(Duration::from_secs(1));
    let interval_stream = IntervalStream::new(interval);
    let mut notification_stream = interval_stream.map(move |_| {
        ValueNotification { value: vec![42], uuid: Uuid::nil() }
    });

    loop {
        tokio::select! {
            Some(data) = notification_stream.next() => {
                let characteristic_uuid = data.uuid;
                let characteristic_value = data.value;
                println!("From characteristic {} received value {:02X?}.", characteristic_uuid, characteristic_value);
            },
            Some(msg) = ws_receiver.next() => {
                on_ws_message(msg?, |characteristic_uuid, characteristic_value| {
                    println!("To characteristic {} sent value {:02X?}.", characteristic_uuid, characteristic_value);
                });
            },
        }
    }
}

async fn wait_for_client_connection() -> Result<WebSocketStream<TcpStream>, Box<dyn Error>> {
    let addr = "127.0.0.1:9001";
    let server = TcpListener::bind(&addr).await.expect("Cannot listen on port 9001.");

    println!("Waiting for connection on addres {}", addr);
    let (stream, _) = server.accept().await?;
    let peer = stream.peer_addr().expect("connected streams should have a peer address");
    println!("We are connected to peer with address {}", peer);
    let ws_stream = accept_async(stream).await.expect("Failed to accept websocket.");
    println!("WebSocket connection successful");
    return Ok(ws_stream);
}

async fn listen_for_updates(controller: &Peripheral, characteristic: &Characteristic) -> Result<(), Box<dyn Error>> {
    controller.subscribe(characteristic).await?;
    // let update_stream = controller.notifications().await?;
    let mut notification_stream = controller.notifications().await?.take(16);
    // Process while the BLE connection is not broken or stopped.
    while let Some(data) = notification_stream.next().await {
        println!(
            "uuid: {:?}, value:{:?}",
            data.uuid, data.value
        );
    }
    controller.unsubscribe(characteristic).await?;
    Ok(())
}

async fn wait_for_motion_controller(central: &Adapter) -> Peripheral {
    // start scanning for devices
    let scan_filter = ScanFilter { services: vec![Uuid::parse_str(DIYMOTIONCONTROLLER_SERVICE_UUID).unwrap()] };
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
