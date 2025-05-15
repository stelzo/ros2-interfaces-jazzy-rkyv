use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetModeRequest {
    pub base_mode: u8,
    pub custom_mode: ::std::string::String,
}

impl SetModeRequest {
    pub const MAV_MODE_PREFLIGHT: u8 = 0;
    pub const MAV_MODE_STABILIZE_DISARMED: u8 = 80;
    pub const MAV_MODE_STABILIZE_ARMED: u8 = 208;
    pub const MAV_MODE_MANUAL_DISARMED: u8 = 64;
    pub const MAV_MODE_MANUAL_ARMED: u8 = 192;
    pub const MAV_MODE_GUIDED_DISARMED: u8 = 88;
    pub const MAV_MODE_GUIDED_ARMED: u8 = 216;
    pub const MAV_MODE_AUTO_DISARMED: u8 = 92;
    pub const MAV_MODE_AUTO_ARMED: u8 = 220;
    pub const MAV_MODE_TEST_DISARMED: u8 = 66;
    pub const MAV_MODE_TEST_ARMED: u8 = 194;
}

impl Default for SetModeRequest {
    fn default() -> Self {
        SetModeRequest {
            base_mode: 0,
            custom_mode: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetModeResponse {
    pub mode_sent: bool,
}

impl Default for SetModeResponse {
    fn default() -> Self {
        SetModeResponse { mode_sent: false }
    }
}

pub struct SetMode;
