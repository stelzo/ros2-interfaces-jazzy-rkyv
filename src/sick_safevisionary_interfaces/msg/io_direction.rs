use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IODirection {
    pub pin_5: u16,
    pub pin_6: u16,
    pub pin_7: u16,
    pub pin_8: u16,
}

impl Default for IODirection {
    fn default() -> Self {
        IODirection {
            pin_5: 0,
            pin_6: 0,
            pin_7: 0,
            pin_8: 0,
        }
    }
}
