use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetJointPropertiesRequest {
    pub joint_name: ::std::string::String,
    pub ode_joint_config: crate::gazebo_msgs::msg::ODEJointProperties,
}

impl Default for SetJointPropertiesRequest {
    fn default() -> Self {
        SetJointPropertiesRequest {
            joint_name: ::std::string::String::new(),
            ode_joint_config: crate::gazebo_msgs::msg::ODEJointProperties::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetJointPropertiesResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetJointPropertiesResponse {
    fn default() -> Self {
        SetJointPropertiesResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetJointProperties;
