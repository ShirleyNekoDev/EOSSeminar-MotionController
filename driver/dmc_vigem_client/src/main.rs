extern crate websocket;

use std::sync::mpsc::channel;
use std::thread;

use dmc::ClientUpdate;
use vigem_client::{XButtons};
use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};

const CONNECTION: &'static str = "ws://127.0.0.1:9001";

fn map_to_i16(value: f32) -> i16 {
    assert!((-1.0..=1.0).contains(&value));
    return if value < 0.0 {
        value.abs() * i16::MIN as f32
    } else {
        value * i16::MAX as f32
    } as i16;
}

pub trait GamepadButtonsExt {
    fn update(&mut self, button: u16, pressed: bool);
    fn is_pressed(&mut self, button: u16) -> bool;
}
impl GamepadButtonsExt for XButtons {
    fn update(&mut self, button: u16, pressed: bool) {
        if pressed {
            self.raw |= button; // set bit
        } else {
            self.raw &= !button; // unset bit
        }
    }
    fn is_pressed(&mut self, button: u16) -> bool {
        return (self.raw & button) > 0;
    }
}

fn main() {
    println!("Connecting to WS {}", CONNECTION);
    let ws_client = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .connect_insecure()
        .unwrap();
    println!("Successfully connected");

    let (mut receiver, mut sender) = ws_client.split().unwrap();
    let (tx, rx) = channel();
    let tx_1 = tx.clone();

    let send_loop = thread::spawn(move || {
        loop {
            // Send loop
            let message = match rx.recv() {
                Ok(m) => m,
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    return;
                }
            };
            println!("Send: {:?}", message);
            match message {
                OwnedMessage::Close(_) => {
                    let _ = sender.send_message(&message);
                    // If it's a close message, just send it and then return.
                    return;
                }
                _ => (),
            }
            // Send the message
            match sender.send_message(&message) {
                Ok(()) => (),
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    let _ = sender.send_message(&Message::close());
                    return;
                }
            }
        }
    });

    let receive_loop = thread::spawn(move || {
        println!("ViGEmBus driver");
        let vigem_client = vigem_client::Client::connect().unwrap();
        println!("Successfully connected");

        println!("Creating virtual controller target");
        let mut controller =
            vigem_client::Xbox360Wired::new(vigem_client, vigem_client::TargetId::XBOX360_WIRED);

        // The input state of the virtual controller
        // https://docs.rs/vigem-client/latest/vigem_client/struct.XGamepad.html
        let mut gamepad = vigem_client::XGamepad {
            ..Default::default()
        };

        // Receive loop
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive Loop: {:?}", e);
                    let _ = tx_1.send(OwnedMessage::Close(None));

                    println!("Disconnecting controller");
                    controller.unplug().unwrap();
                    return;
                }
            };
            match message {
                OwnedMessage::Close(_) => {
                    // Got a close message, so send a close message and return
                    let _ = tx_1.send(OwnedMessage::Close(None));

                    println!("Disconnecting controller");
                    controller.unplug().unwrap();
                    return;
                }
                OwnedMessage::Ping(data) => {
                    match tx_1.send(OwnedMessage::Pong(data)) {
                        // Send a pong in response
                        Ok(()) => (),
                        Err(e) => {
                            println!("Receive Loop: {:?}", e);

                            println!("Disconnecting controller");
                            controller.unplug().unwrap();
                            return;
                        }
                    }
                }
                OwnedMessage::Text(message) => {
                    let updates =
                        serde_json::from_str::<Vec<ClientUpdate>>(message.as_str()).unwrap();

                    for update in updates {
                        println!("Status change: {:?}", update);
                        match update {
                            ClientUpdate::Connected => {
                                println!("Connecting controller");
                                // Plugin the virtual controller
                                controller.plugin().unwrap();
                                // Wait for the virtual controller to be ready to accept updates
                                controller.wait_ready().unwrap();
                                println!("Controller ready");
                            }
                            ClientUpdate::Disconnected => {
                                println!("Disconnecting controller");
                                controller.unplug().unwrap();
                                println!("Controller removed");
                            }
                            ClientUpdate::JoystickMoved { x, y } => {
                                assert!(controller.is_attached());
                                println!("Controller right joystick update: x={}, y={}", x, y);
                                gamepad.thumb_rx = map_to_i16(x);
                                gamepad.thumb_ry = map_to_i16(y);
                                controller.update(&gamepad).unwrap();
                            },
                            _ => {
                                let mut update_button = |button: u16, pressed: bool| {
                                    assert!(controller.is_attached());
                                    gamepad.buttons.update(button, pressed);
                                    controller.update(&gamepad).unwrap();
                                };
                                match update {
                                    ClientUpdate::ButtonADown => update_button(XButtons::A, true),
                                    ClientUpdate::ButtonAUp => update_button(XButtons::A, false),
                                    ClientUpdate::ButtonBDown => update_button(XButtons::B, true),
                                    ClientUpdate::ButtonBUp => update_button(XButtons::B, false),
                                    ClientUpdate::ButtonMenuDown => update_button(XButtons::START, true),
                                    ClientUpdate::ButtonMenuUp => update_button(XButtons::START, false),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                _ => {
                    println!("Disconnecting controller");
                    controller.unplug().unwrap();

                    panic!("Unhandled message type: {:?}", message);
                }
            }
        }
    });

    println!("Waiting for child threads to exit");

    let _ = send_loop.join();
    let _ = receive_loop.join();

    println!("Exited");
}
