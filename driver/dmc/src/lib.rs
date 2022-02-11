use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub enum ClientCommand {
    LedSet {
        r: u8,
        g: u8,
        b: u8,
    },
    RumbleStart,
    RumbleStop,
    RumbleBurst {
        length: u8, // * 10ms (therefore 1 => 10ms, 255 => 2550ms)
    },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub enum ClientUpdate {
    Connected,
    Disconnected,
    BatteryStatusChanged { charge: u8 },
    ButtonADown,
    ButtonAUp,
    ButtonBDown,
    ButtonBUp,
    ButtonMenuDown,
    ButtonMenuUp,
    JoystickMoved { x: f32, y: f32 },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
