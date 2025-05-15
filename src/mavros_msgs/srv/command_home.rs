use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandHomeRequest {
    pub current_gps: bool,
    pub yaw: f32,
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
}

impl Default for CommandHomeRequest {
    fn default() -> Self {
        CommandHomeRequest {
            current_gps: false,
            yaw: 0.0,
            latitude: 0.0,
            longitude: 0.0,
            altitude: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandHomeResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for CommandHomeResponse {
    fn default() -> Self {
        CommandHomeResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct CommandHome;
