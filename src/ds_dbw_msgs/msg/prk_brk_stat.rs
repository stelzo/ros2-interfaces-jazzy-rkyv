use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PrkBrkStat {
    pub value: u8,
}

impl PrkBrkStat {
    pub const UNKNOWN: u8 = 0;
    pub const ON: u8 = 1;
    pub const OFF: u8 = 2;
    pub const TRANSITION: u8 = 3;
}

impl Default for PrkBrkStat {
    fn default() -> Self {
        PrkBrkStat { value: 0 }
    }
}
