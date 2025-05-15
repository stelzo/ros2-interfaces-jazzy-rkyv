use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TeleportRelativeRequest {
    pub linear: f32,
    pub angular: f32,
}

impl Default for TeleportRelativeRequest {
    fn default() -> Self {
        TeleportRelativeRequest {
            linear: 0.0,
            angular: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TeleportRelativeResponse {}

impl Default for TeleportRelativeResponse {
    fn default() -> Self {
        TeleportRelativeResponse {}
    }
}

pub struct TeleportRelative;
