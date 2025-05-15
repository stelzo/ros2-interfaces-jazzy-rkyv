use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GimbalManagerConfigureRequest {
    pub sysid_primary: i16,
    pub compid_primary: i16,
    pub sysid_secondary: i16,
    pub compid_secondary: i16,
    pub gimbal_device_id: u8,
}

impl Default for GimbalManagerConfigureRequest {
    fn default() -> Self {
        GimbalManagerConfigureRequest {
            sysid_primary: 0,
            compid_primary: 0,
            sysid_secondary: 0,
            compid_secondary: 0,
            gimbal_device_id: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GimbalManagerConfigureResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalManagerConfigureResponse {
    fn default() -> Self {
        GimbalManagerConfigureResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct GimbalManagerConfigure;
