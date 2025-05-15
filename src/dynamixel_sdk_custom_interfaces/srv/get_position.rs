use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPositionRequest {
    pub id: u8,
}

impl Default for GetPositionRequest {
    fn default() -> Self {
        GetPositionRequest { id: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPositionResponse {
    pub position: i32,
}

impl Default for GetPositionResponse {
    fn default() -> Self {
        GetPositionResponse { position: 0 }
    }
}

pub struct GetPosition;
