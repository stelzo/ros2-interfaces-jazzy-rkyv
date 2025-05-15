use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MountConfigureRequest {
    pub header: crate::std_msgs::msg::Header,
    pub mode: u8,
    pub stabilize_roll: bool,
    pub stabilize_pitch: bool,
    pub stabilize_yaw: bool,
    pub roll_input: u8,
    pub pitch_input: u8,
    pub yaw_input: u8,
}

impl MountConfigureRequest {
    pub const MODE_RETRACT: u8 = 0;
    pub const MODE_NEUTRAL: u8 = 1;
    pub const MODE_MAVLINK_TARGETING: u8 = 2;
    pub const MODE_RC_TARGETING: u8 = 3;
    pub const MODE_GPS_POINT: u8 = 4;
    pub const INPUT_ANGLE_BODY_FRAME: u8 = 0;
    pub const INPUT_ANGULAR_RATE: u8 = 1;
    pub const INPUT_ANGLE_ABSOLUTE_FRAME: u8 = 2;
}

impl Default for MountConfigureRequest {
    fn default() -> Self {
        MountConfigureRequest {
            header: crate::std_msgs::msg::Header::default(),
            mode: 0,
            stabilize_roll: false,
            stabilize_pitch: false,
            stabilize_yaw: false,
            roll_input: 0,
            pitch_input: 0,
            yaw_input: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MountConfigureResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for MountConfigureResponse {
    fn default() -> Self {
        MountConfigureResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct MountConfigure;
