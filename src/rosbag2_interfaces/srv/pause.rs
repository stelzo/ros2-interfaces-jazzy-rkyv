use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PauseRequest {}

impl Default for PauseRequest {
    fn default() -> Self {
        PauseRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PauseResponse {}

impl Default for PauseResponse {
    fn default() -> Self {
        PauseResponse {}
    }
}

pub struct Pause;
