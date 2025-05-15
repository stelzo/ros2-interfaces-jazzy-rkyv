use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COHeartbeatIDRequest {
    pub nodeid: u8,
    pub heartbeat: u16,
}

impl Default for COHeartbeatIDRequest {
    fn default() -> Self {
        COHeartbeatIDRequest {
            nodeid: 0,
            heartbeat: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct COHeartbeatIDResponse {
    pub success: bool,
}

impl Default for COHeartbeatIDResponse {
    fn default() -> Self {
        COHeartbeatIDResponse { success: false }
    }
}

pub struct COHeartbeatID;
