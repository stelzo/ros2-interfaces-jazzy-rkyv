use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TeleportAbsoluteRequest {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Default for TeleportAbsoluteRequest {
    fn default() -> Self {
        TeleportAbsoluteRequest {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TeleportAbsoluteResponse {}

impl Default for TeleportAbsoluteResponse {
    fn default() -> Self {
        TeleportAbsoluteResponse {}
    }
}

pub struct TeleportAbsolute;
