use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HeadlightCtrlHigh {
    pub value: u8,
}

impl HeadlightCtrlHigh {
    pub const UNKNOWN: u8 = 0;
    pub const OFF: u8 = 1;
    pub const ON: u8 = 2;
    pub const AUTO: u8 = 3;
    pub const FLASH: u8 = 4;
}

impl Default for HeadlightCtrlHigh {
    fn default() -> Self {
        HeadlightCtrlHigh { value: 0 }
    }
}
