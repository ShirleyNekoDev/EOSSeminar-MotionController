use dmc::ClientCommand;
use bincode;
use std::io::Write;

fn write_to_stdout(cmd: &ClientCommand) {
    let raw_data = bincode::serialize(&cmd).unwrap();
    let raw_data_slice = raw_data.as_slice();
    std::io::stdout().write_all(raw_data_slice).unwrap();
}

fn main() {
    write_to_stdout(&ClientCommand::LedSet { r: 32, g: 16, b: 8 });
    write_to_stdout(&ClientCommand::RumbleStart);
    write_to_stdout(&ClientCommand::RumbleStop);
    write_to_stdout(&ClientCommand::RumbleBurst { length: 4096 });
}
