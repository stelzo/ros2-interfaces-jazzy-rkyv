use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandTOLRequest {
    pub min_pitch: f32,
    pub yaw: f32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

impl Default for CommandTOLRequest {
    fn default() -> Self {
        CommandTOLRequest {
            min_pitch: 0.0,
            yaw: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandTOLResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandTOLResponse {
    fn default() -> Self {
        CommandTOLResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct CommandTOL;
