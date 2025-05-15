use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SystemState {
    pub value: u8,
}

impl SystemState {
    pub const MANUAL: u8 = 0;
    pub const READY: u8 = 1;
    pub const ACTIVE: u8 = 2;
    pub const FAULT: u8 = 7;
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState { value: 0 }
    }
}
