
use std::fs::OpenOptions;
use memmap::MmapMut;
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
struct AccelerometerData {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    let file = OpenOptions::new()
                .write(true)
                .read(true)
                .create(true)
                .open("/tmp/dmtrs")
                .unwrap();
    file.set_len(8).unwrap();

    let mut map = unsafe { MmapMut::map_mut(&file).unwrap() };
    let mut acc_data = AccelerometerData { x: 0.24, y: 1.0, z: -0.3};
    loop {
        std::thread::sleep(std::time::Duration::from_millis(10));
        let enc: Vec<u8> = bincode::serialize(&acc_data).unwrap();
        map[0..0+enc.len()].copy_from_slice(enc.leak());
        acc_data.x += 0.01;
        acc_data.y -= 0.01;
        acc_data.z += 0.014;
    }
}
