use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WarmStartRequest {
    pub reset_type: u8,
}

impl Default for WarmStartRequest {
    fn default() -> Self {
        WarmStartRequest { reset_type: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WarmStartResponse {}

impl Default for WarmStartResponse {
    fn default() -> Self {
        WarmStartResponse {}
    }
}

pub struct WarmStart;
