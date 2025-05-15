use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UUID {
    pub uuid: [u8; 16],
}

impl Default for UUID {
    fn default() -> Self {
        UUID { uuid: [0; 16] }
    }
}
