use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLinkPropertiesRequest {
    pub link_name: ::std::string::String,
    pub com: crate::geometry_msgs::msg::Pose,
    pub gravity_mode: bool,
    pub mass: f64,
    pub ixx: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyy: f64,
    pub iyz: f64,
    pub izz: f64,
}

impl Default for SetLinkPropertiesRequest {
    fn default() -> Self {
        SetLinkPropertiesRequest {
            link_name: ::std::string::String::new(),
            com: crate::geometry_msgs::msg::Pose::default(),
            gravity_mode: false,
            mass: 0.0,
            ixx: 0.0,
            ixy: 0.0,
            ixz: 0.0,
            iyy: 0.0,
            iyz: 0.0,
            izz: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLinkPropertiesResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetLinkPropertiesResponse {
    fn default() -> Self {
        SetLinkPropertiesResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetLinkProperties;
