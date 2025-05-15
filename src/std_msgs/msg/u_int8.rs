use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UInt8 {
    pub data: u8,
}

impl Default for UInt8 {
    fn default() -> Self {
        UInt8 { data: 0 }
    }
}
