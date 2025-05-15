use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsPausedRequest {}

impl Default for IsPausedRequest {
    fn default() -> Self {
        IsPausedRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsPausedResponse {
    pub paused: bool,
}

impl Default for IsPausedResponse {
    fn default() -> Self {
        IsPausedResponse { paused: false }
    }
}

pub struct IsPaused;
