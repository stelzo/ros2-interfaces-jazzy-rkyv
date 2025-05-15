use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GimbalManagerCameraTrackRequest {
    pub mode: u8,
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub top_left_x: f32,
    pub top_left_y: f32,
    pub bottom_right_x: f32,
    pub bottom_right_y: f32,
}

impl GimbalManagerCameraTrackRequest {
    pub const CAMERA_TRACK_MODE_POINT: u8 = 0;
    pub const CAMERA_TRACK_MODE_RECTANGLE: u8 = 1;
    pub const CAMERA_TRACK_MODE_STOP_TRACKING: u8 = 2;
}

impl Default for GimbalManagerCameraTrackRequest {
    fn default() -> Self {
        GimbalManagerCameraTrackRequest {
            mode: 0,
            x: 0.0,
            y: 0.0,
            radius: 0.0,
            top_left_x: 0.0,
            top_left_y: 0.0,
            bottom_right_x: 0.0,
            bottom_right_y: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GimbalManagerCameraTrackResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalManagerCameraTrackResponse {
    fn default() -> Self {
        GimbalManagerCameraTrackResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct GimbalManagerCameraTrack;
