use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SensorData {
    pub distance: u8,
    pub warn: bool,
    pub alarm: bool,
    pub active: bool,
}

impl Default for SensorData {
    fn default() -> Self {
        SensorData {
            distance: 0,
            warn: false,
            alarm: false,
            active: false,
        }
    }
}
