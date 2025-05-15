use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ResetPoseRequest {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for ResetPoseRequest {
    fn default() -> Self {
        ResetPoseRequest {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            roll: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ResetPoseResponse {}

impl Default for ResetPoseResponse {
    fn default() -> Self {
        ResetPoseResponse {}
    }
}

pub struct ResetPose;
