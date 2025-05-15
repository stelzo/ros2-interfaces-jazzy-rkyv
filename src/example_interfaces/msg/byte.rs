use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Byte {
    pub data: u8,
}

impl Default for Byte {
    fn default() -> Self {
        Byte { data: 0 }
    }
}
