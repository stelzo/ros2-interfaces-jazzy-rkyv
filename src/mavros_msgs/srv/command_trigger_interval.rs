use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandTriggerIntervalRequest {
    pub cycle_time: f32,
    pub integration_time: f32,
}

impl Default for CommandTriggerIntervalRequest {
    fn default() -> Self {
        CommandTriggerIntervalRequest {
            cycle_time: 0.0,
            integration_time: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandTriggerIntervalResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTriggerIntervalResponse {
    fn default() -> Self {
        CommandTriggerIntervalResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct CommandTriggerInterval;
