use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UInt16 {
    pub data: u16,
}

impl Default for UInt16 {
    fn default() -> Self {
        UInt16 { data: 0 }
    }
}
