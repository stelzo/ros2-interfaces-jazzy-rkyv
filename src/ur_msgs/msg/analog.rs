use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Analog {
    pub pin: u8,
    pub domain: u8,
    pub state: f32,
}

impl Analog {
    pub const CURRENT: u8 = 0;
    pub const VOLTAGE: u8 = 1;
}

impl Default for Analog {
    fn default() -> Self {
        Analog {
            pin: 0,
            domain: 0,
            state: 0.0,
        }
    }
}
