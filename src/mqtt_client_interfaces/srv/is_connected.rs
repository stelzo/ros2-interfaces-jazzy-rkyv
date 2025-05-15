use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsConnectedRequest {}

impl Default for IsConnectedRequest {
    fn default() -> Self {
        IsConnectedRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsConnectedResponse {
    pub connected: bool,
}

impl Default for IsConnectedResponse {
    fn default() -> Self {
        IsConnectedResponse { connected: false }
    }
}

pub struct IsConnected;
