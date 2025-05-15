use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Uint8Array3TestMessage {
    pub data: [u8; 3],
}

impl Default for Uint8Array3TestMessage {
    fn default() -> Self {
        Uint8Array3TestMessage { data: [0; 3] }
    }
}
