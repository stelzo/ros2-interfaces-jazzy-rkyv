use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PrkBrkCmd {
    pub value: u8,
}

impl PrkBrkCmd {
    pub const NONE: u8 = 0;
    pub const ON: u8 = 1;
    pub const OFF: u8 = 2;
}

impl Default for PrkBrkCmd {
    fn default() -> Self {
        PrkBrkCmd { value: 0 }
    }
}
