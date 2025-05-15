use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TogglePausedRequest {}

impl Default for TogglePausedRequest {
    fn default() -> Self {
        TogglePausedRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TogglePausedResponse {}

impl Default for TogglePausedResponse {
    fn default() -> Self {
        TogglePausedResponse {}
    }
}

pub struct TogglePaused;
