use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientCommand {
    LedSet { r: u8, g: u8, b: u8 },
    RumbleStart,
    RumbleStop,
    RumbleBurst { length: i16 },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientUpdate {
    BatteryStatusChanged { charge: u8 },
    ButtonADown,
    ButtonAUp,
    ButtonBDown,
    ButtonBUp,
    ButtonMenuDown,
    ButtonMenuUp,
    JoystickMoved { x: f32, y: f32 }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
