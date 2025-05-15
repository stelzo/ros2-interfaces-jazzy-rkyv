use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ControlModeCommandRequest {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub mode: u8,
}

impl ControlModeCommandRequest {
    pub const NO_COMMAND: u8 = 0;
    pub const AUTONOMOUS: u8 = 1;
    pub const AUTONOMOUS_STEER_ONLY: u8 = 2;
    pub const AUTONOMOUS_VELOCITY_ONLY: u8 = 3;
    pub const MANUAL: u8 = 4;
}

impl Default for ControlModeCommandRequest {
    fn default() -> Self {
        ControlModeCommandRequest {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            mode: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ControlModeCommandResponse {
    pub success: bool,
}

impl Default for ControlModeCommandResponse {
    fn default() -> Self {
        ControlModeCommandResponse { success: false }
    }
}

pub struct ControlModeCommand;
