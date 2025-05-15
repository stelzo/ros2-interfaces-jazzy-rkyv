use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandTriggerControlRequest {
    pub trigger_enable: bool,
    pub sequence_reset: bool,
    pub trigger_pause: bool,
}

impl Default for CommandTriggerControlRequest {
    fn default() -> Self {
        CommandTriggerControlRequest {
            trigger_enable: false,
            sequence_reset: false,
            trigger_pause: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandTriggerControlResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTriggerControlResponse {
    fn default() -> Self {
        CommandTriggerControlResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct CommandTriggerControl;
