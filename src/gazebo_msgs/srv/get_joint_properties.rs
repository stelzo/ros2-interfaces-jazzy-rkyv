use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetJointPropertiesRequest {
    pub joint_name: ::std::string::String,
}

impl Default for GetJointPropertiesRequest {
    fn default() -> Self {
        GetJointPropertiesRequest {
            joint_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetJointPropertiesResponse {
    pub r#type: u8,
    pub damping: Vec<f64>,
    pub position: Vec<f64>,
    pub rate: Vec<f64>,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl GetJointPropertiesResponse {
    pub const REVOLUTE: u8 = 0;
    pub const CONTINUOUS: u8 = 1;
    pub const PRISMATIC: u8 = 2;
    pub const FIXED: u8 = 3;
    pub const BALL: u8 = 4;
    pub const UNIVERSAL: u8 = 5;
}

impl Default for GetJointPropertiesResponse {
    fn default() -> Self {
        GetJointPropertiesResponse {
            r#type: 0,
            damping: Vec::new(),
            position: Vec::new(),
            rate: Vec::new(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct GetJointProperties;
