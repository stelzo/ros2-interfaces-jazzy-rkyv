use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandLongRequest {
    pub broadcast: bool,
    pub command: u16,
    pub confirmation: u8,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub param5: f32,
    pub param6: f32,
    pub param7: f32,
}

impl Default for CommandLongRequest {
    fn default() -> Self {
        CommandLongRequest {
            broadcast: false,
            command: 0,
            confirmation: 0,
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            param5: 0.0,
            param6: 0.0,
            param7: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandLongResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandLongResponse {
    fn default() -> Self {
        CommandLongResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct CommandLong;
