use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandAckRequest {
    pub command: u16,
    pub result: u8,
    pub progress: u8,
    pub result_param2: u32,
}

impl Default for CommandAckRequest {
    fn default() -> Self {
        CommandAckRequest {
            command: 0,
            result: 0,
            progress: 0,
            result_param2: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandAckResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandAckResponse {
    fn default() -> Self {
        CommandAckResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct CommandAck;
