use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AudioData {
    pub data: Vec<u8>,
}

impl Default for AudioData {
    fn default() -> Self {
        AudioData { data: Vec::new() }
    }
}
