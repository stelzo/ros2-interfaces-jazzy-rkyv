use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Uint8ArrayTestMessage {
    pub data: Vec<u8>,
}

impl Default for Uint8ArrayTestMessage {
    fn default() -> Self {
        Uint8ArrayTestMessage { data: Vec::new() }
    }
}
