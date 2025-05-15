use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandIntRequest {
    pub broadcast: bool,
    pub frame: u8,
    pub command: u16,
    pub current: u8,
    pub autocontinue: u8,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub param4: f32,
    pub x: i32,
    pub y: i32,
    pub z: f32,
}

impl Default for CommandIntRequest {
    fn default() -> Self {
        CommandIntRequest {
            broadcast: false,
            frame: 0,
            command: 0,
            current: 0,
            autocontinue: 0,
            param1: 0.0,
            param2: 0.0,
            param3: 0.0,
            param4: 0.0,
            x: 0,
            y: 0,
            z: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandIntResponse {
    pub success: bool,
}

impl Default for CommandIntResponse {
    fn default() -> Self {
        CommandIntResponse { success: false }
    }
}

pub struct CommandInt;
