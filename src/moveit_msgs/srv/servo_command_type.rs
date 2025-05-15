use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServoCommandTypeRequest {
    pub command_type: i8,
}

impl ServoCommandTypeRequest {
    pub const JOINT_JOG: i8 = 0;
    pub const TWIST: i8 = 1;
    pub const POSE: i8 = 2;
}

impl Default for ServoCommandTypeRequest {
    fn default() -> Self {
        ServoCommandTypeRequest { command_type: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServoCommandTypeResponse {
    pub success: bool,
}

impl Default for ServoCommandTypeResponse {
    fn default() -> Self {
        ServoCommandTypeResponse { success: false }
    }
}

pub struct ServoCommandType;
