use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Digital {
    pub pin: u8,
    pub state: bool,
}

impl Default for Digital {
    fn default() -> Self {
        Digital {
            pin: 0,
            state: false,
        }
    }
}
