use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MessageIntervalRequest {
    pub message_id: u32,
    pub message_rate: f32,
}

impl Default for MessageIntervalRequest {
    fn default() -> Self {
        MessageIntervalRequest {
            message_id: 0,
            message_rate: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MessageIntervalResponse {
    pub success: bool,
}

impl Default for MessageIntervalResponse {
    fn default() -> Self {
        MessageIntervalResponse { success: false }
    }
}

pub struct MessageInterval;
