use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StopRequest {}

impl Default for StopRequest {
    fn default() -> Self {
        StopRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StopResponse {}

impl Default for StopResponse {
    fn default() -> Self {
        StopResponse {}
    }
}

pub struct Stop;
