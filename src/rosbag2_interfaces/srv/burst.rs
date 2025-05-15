use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BurstRequest {
    pub num_messages: u64,
}

impl Default for BurstRequest {
    fn default() -> Self {
        BurstRequest { num_messages: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BurstResponse {
    pub actually_burst: u64,
}

impl Default for BurstResponse {
    fn default() -> Self {
        BurstResponse { actually_burst: 0 }
    }
}

pub struct Burst;
