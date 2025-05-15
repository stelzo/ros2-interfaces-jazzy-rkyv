use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CONodeRequest {
    pub nodeid: u8,
}

impl Default for CONodeRequest {
    fn default() -> Self {
        CONodeRequest { nodeid: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CONodeResponse {
    pub success: bool,
}

impl Default for CONodeResponse {
    fn default() -> Self {
        CONodeResponse { success: false }
    }
}

pub struct CONode;
