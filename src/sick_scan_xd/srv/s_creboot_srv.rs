use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SCrebootSrvRequest {}

impl Default for SCrebootSrvRequest {
    fn default() -> Self {
        SCrebootSrvRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SCrebootSrvResponse {
    pub success: bool,
}

impl Default for SCrebootSrvResponse {
    fn default() -> Self {
        SCrebootSrvResponse { success: false }
    }
}

pub struct SCrebootSrv;
