use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PlayNextRequest {}

impl Default for PlayNextRequest {
    fn default() -> Self {
        PlayNextRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PlayNextResponse {
    pub success: bool,
}

impl Default for PlayNextResponse {
    fn default() -> Self {
        PlayNextResponse { success: false }
    }
}

pub struct PlayNext;
