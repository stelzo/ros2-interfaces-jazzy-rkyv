use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UInt32 {
    pub data: u32,
}

impl Default for UInt32 {
    fn default() -> Self {
        UInt32 { data: 0 }
    }
}
