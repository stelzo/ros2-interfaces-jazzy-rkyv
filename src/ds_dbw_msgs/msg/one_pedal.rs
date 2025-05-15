use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OnePedal {
    pub value: u8,
}

impl OnePedal {
    pub const UNKNOWN: u8 = 0;
    pub const OFF: u8 = 1;
    pub const ON: u8 = 2;
    pub const FAULT: u8 = 3;
}

impl Default for OnePedal {
    fn default() -> Self {
        OnePedal { value: 0 }
    }
}
