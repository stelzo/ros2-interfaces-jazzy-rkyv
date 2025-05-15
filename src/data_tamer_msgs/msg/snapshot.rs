use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Snapshot {
    pub timestamp_nsec: u64,
    pub schema_hash: u64,
    pub active_mask: Vec<u8>,
    pub payload: Vec<u8>,
}

impl Default for Snapshot {
    fn default() -> Self {
        Snapshot {
            timestamp_nsec: 0,
            schema_hash: 0,
            active_mask: Vec::new(),
            payload: Vec::new(),
        }
    }
}
