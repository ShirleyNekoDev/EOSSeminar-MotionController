use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientCommand {
    LedSet { r: u8, g: u8, b: u8 },
    RumbleStart,
    RumbleStop,
    RumbleBurst { length: u32 },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientUpdate {

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
