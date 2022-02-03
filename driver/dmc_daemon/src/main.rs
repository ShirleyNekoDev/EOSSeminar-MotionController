use btleplug::api::{Central, Manager as _, Peripheral as _, Characteristic, ScanFilter, ValueNotification};
use btleplug::platform::{Adapter, Manager, Peripheral};
use tokio_stream::wrappers::IntervalStream;
use std::error::Error;
use std::option::Option;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use tokio::time;
use std::marker::Copy;
use uuid::Uuid;
use futures::stream::StreamExt;
use futures::sink::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, WebSocketStream, tungstenite::Error as TError};
use tungstenite::Message;

use dmc::{ClientCommand, ClientUpdate};

#[derive(Clone,Copy,PartialEq,Debug)]
enum ButtonState {
    UP,
    DOWN,
}

#[derive(Clone,Copy,PartialEq,Debug)]
struct ControllerState {
    battery_level: u8,
    joystick_state: JoystickState,
    button_a_state: ButtonState,
    button_b_state: ButtonState,
    button_menu_state: ButtonState,
}

impl ControllerState {
    // Returns whether or not the state was updated.
    fn button_a_state_transition(&mut self, new_value: bool) -> Option<ClientUpdate> {
        if new_value ^ (self.button_a_state == ButtonState::DOWN) {
            // If the value changed.
            if new_value {
                self.button_a_state = ButtonState::DOWN;
                return Some(ClientUpdate::ButtonADown)
            } else {
                self.button_a_state = ButtonState::UP;
                return Some(ClientUpdate::ButtonAUp)
            }
        }
        // If the value did not change.
        None
    }

    // Returns whether or not the state was updated.
    fn button_b_state_transition(&mut self, new_value: bool) -> Option<ClientUpdate> {
        if new_value ^ (self.button_b_state == ButtonState::DOWN) {
            // If the value changed.
            if new_value {
                self.button_b_state = ButtonState::DOWN;
                return Some(ClientUpdate::ButtonBDown)
            } else {
                self.button_b_state = ButtonState::UP;
                return Some(ClientUpdate::ButtonBUp)
            }
        }
        // If the value did not change.
        None
    }

    // Returns whether or not the state was updated.
    fn button_menu_state_transition(&mut self, new_value: bool) -> Option<ClientUpdate> {
        if new_value ^ (self.button_menu_state == ButtonState::DOWN) {
            // If the value changed.
            if new_value {
                self.button_menu_state = ButtonState::DOWN;
                return Some(ClientUpdate::ButtonMenuDown)
            } else {
                self.button_menu_state = ButtonState::UP;
                return Some(ClientUpdate::ButtonMenuUp)
            }
        }
        // If the value did not change.
        None
    }
}


impl ControllerState {
    fn new() -> Self {
        ControllerState {
            battery_level: 255,
            joystick_state: JoystickState::new(),
            button_a_state: ButtonState::UP,
            button_b_state: ButtonState::UP,
            button_menu_state: ButtonState::UP,
        }
    }
}

#[derive(Clone,Copy,PartialEq,Debug)]
struct JoystickState {
    x: f32,
    y: f32,
}

fn unpack_float(packed_f16: u16) -> f32 {
    let h: u32 = packed_f16.into();
    f32::from_bits(((h & 0x8000) << 16) | (((h&0x7c00)+0x1C000)<<13) | ((h&0x03FF)<<13))
}

impl JoystickState {
    fn new() -> Self {
        JoystickState { x: 0.0, y: 0.0 }
    }
    fn from_raw_f16s(raw_x: u16, raw_y: u16) -> Self {
        JoystickState { x: unpack_float(raw_x), y: unpack_float(raw_y) }
    }
}

impl Into<ClientUpdate> for JoystickState {
    fn into(self) -> ClientUpdate {
        ClientUpdate::JoystickMoved { x: self.x, y: self.y }
    }
}

const DIYMOTIONCONTROLLER_SERVICE_UUID: &str = "328c9225-877f-4189-89a8-b50bb21b02ae";
const CLASSIC_CONTROL_CHARACTERISTIC_UUID: &str = "0385fe9d-56a6-40a4-b055-9b610cfcfe0c";

fn on_ws_message(msg: Message, _controller_write_method: fn(Uuid, Vec<u8>)) {
    if let Message::Binary(data) = msg {
        match bincode::deserialize::<ClientCommand>(&data).unwrap() {
            ClientCommand::LedSet { r: _, g: _, b: _ } => {

            },
            ClientCommand::RumbleStop => {

            },
            ClientCommand::RumbleStart => {

            },
            ClientCommand::RumbleBurst { length: _ } => {

            },
        }
    }
}


// This function reads the classic control characteristic's raw data and updates the controller
// state.
fn build_classic_control_updates(controller_state: &mut ControllerState, value: &Vec<u8>) -> Option<Vec<u8>> {
    assert!(value.len() == 5, "ClassicControlsCharacteristic's value must be 5 bytes.");
    let mut update_chain = Vec::<u8>::new();


    let raw_x: u16 = u16::from_le_bytes(value.as_slice()[0..2].try_into().unwrap());
    let raw_y: u16 = u16::from_le_bytes(value.as_slice()[2..4].try_into().unwrap());
    let button_byte = value[4];
    let button_a = (button_byte & 1u8 << 7) > 0;
    let button_b = (button_byte & 1u8 << 6) > 0;
    let button_menu = (button_byte & 1u8 << 5) > 0;

    // Update joystick data
    let joystick_data_read = JoystickState::from_raw_f16s(raw_x, raw_y);
    if joystick_data_read != controller_state.joystick_state {
        controller_state.joystick_state = joystick_data_read;
        let update: ClientUpdate = controller_state.joystick_state.into();
        let mut raw_message = bincode::serialize(&update).unwrap();
        update_chain.append(&mut raw_message);
    }

    // Update button data
    if let Some(update) = controller_state.button_a_state_transition(button_a) {
        update_chain.append(&mut bincode::serialize(&update).unwrap());
    }
    if let Some(update) = controller_state.button_b_state_transition(button_b) {
        update_chain.append(&mut bincode::serialize(&update).unwrap());
    }
    if let Some(update) = controller_state.button_menu_state_transition(button_menu) {
        update_chain.append(&mut bincode::serialize(&update).unwrap());
    }

    if update_chain.is_empty() {
        None
    } else {
        Some(update_chain)
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
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

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
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        ValueNotification { value: ts.to_ne_bytes()[0..5].to_vec(), uuid: Uuid::parse_str(CLASSIC_CONTROL_CHARACTERISTIC_UUID).unwrap() }
    });

    let mut controller_state = ControllerState::new();

    loop {
        tokio::select! {
            Some(data) = notification_stream.next() => {
                let characteristic_uuid = data.uuid;
                let characteristic_value = data.value;
                if characteristic_uuid == Uuid::parse_str(CLASSIC_CONTROL_CHARACTERISTIC_UUID).unwrap() {
                    if let Some(chain) = build_classic_control_updates(&mut controller_state, &characteristic_value) {
                        ws_sender.send(Message::binary(chain)).await?;
                    }
                    println!("new controller state: {:?}", controller_state);
                }
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
