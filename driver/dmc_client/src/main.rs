use dmc::{ClientCommand, ClientUpdate};
use serde::Serialize;
use serde_json::to_string;
use std::io::Write;

fn write_to_stdout<T: Serialize>(cmd: &T) {
    let raw_data = to_string(&cmd).unwrap();
    let raw_data_slice = raw_data.as_bytes();
    std::io::stdout().write_all(raw_data_slice).unwrap();
}

fn main() {
    write_to_stdout(&ClientCommand::LedSet { r: 32, g: 16, b: 8 });
    write_to_stdout(&ClientCommand::RumbleStart);
    write_to_stdout(&ClientCommand::RumbleStop);
    write_to_stdout(&ClientCommand::RumbleBurst { length: 50 });
    write_to_stdout(&ClientUpdate::ButtonADown);
    write_to_stdout(&ClientUpdate::JoystickMoved { x: 0.0, y: 1.0 });
}
