use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CompressedMsg {
    pub data: Vec<u8>,
}

impl Default for CompressedMsg {
    fn default() -> Self {
        CompressedMsg { data: Vec::new() }
    }
}
