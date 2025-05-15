use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPhysicsPropertiesRequest {
    pub time_step: f64,
    pub max_update_rate: f64,
    pub gravity: crate::geometry_msgs::msg::Vector3,
    pub ode_config: crate::gazebo_msgs::msg::ODEPhysics,
}

impl Default for SetPhysicsPropertiesRequest {
    fn default() -> Self {
        SetPhysicsPropertiesRequest {
            time_step: 0.0,
            max_update_rate: 0.0,
            gravity: crate::geometry_msgs::msg::Vector3::default(),
            ode_config: crate::gazebo_msgs::msg::ODEPhysics::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPhysicsPropertiesResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetPhysicsPropertiesResponse {
    fn default() -> Self {
        SetPhysicsPropertiesResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetPhysicsProperties;
