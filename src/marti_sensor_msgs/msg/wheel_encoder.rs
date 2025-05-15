use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WheelEncoder {
    pub frequency: f64,
    pub directional: bool,
    pub id: u8,
}

impl Default for WheelEncoder {
    fn default() -> Self {
        WheelEncoder {
            frequency: 0.0,
            directional: false,
            id: 0,
        }
    }
}
