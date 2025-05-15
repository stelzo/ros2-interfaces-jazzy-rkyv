use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GimbalManagerPitchyawRequest {
    pub pitch: f32,
    pub yaw: f32,
    pub pitch_rate: f32,
    pub yaw_rate: f32,
    pub flags: u32,
    pub gimbal_device_id: u8,
}

impl GimbalManagerPitchyawRequest {
    pub const GIMBAL_MANAGER_FLAGS_RETRACT: u32 = 1;
    pub const GIMBAL_MANAGER_FLAGS_NEUTRAL: u32 = 2;
    pub const GIMBAL_MANAGER_FLAGS_ROLL_LOCK: u32 = 4;
    pub const GIMBAL_MANAGER_FLAGS_PITCH_LOCK: u32 = 8;
    pub const GIMBAL_MANAGER_FLAGS_YAW_LOCK: u32 = 16;
}

impl Default for GimbalManagerPitchyawRequest {
    fn default() -> Self {
        GimbalManagerPitchyawRequest {
            pitch: 0.0,
            yaw: 0.0,
            pitch_rate: 0.0,
            yaw_rate: 0.0,
            flags: 0,
            gimbal_device_id: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GimbalManagerPitchyawResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalManagerPitchyawResponse {
    fn default() -> Self {
        GimbalManagerPitchyawResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct GimbalManagerPitchyaw;
