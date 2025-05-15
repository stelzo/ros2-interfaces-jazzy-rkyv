use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UInt64 {
    pub data: u64,
}

impl Default for UInt64 {
    fn default() -> Self {
        UInt64 { data: 0 }
    }
}
