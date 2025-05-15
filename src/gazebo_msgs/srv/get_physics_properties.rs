use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPhysicsPropertiesRequest {}

impl Default for GetPhysicsPropertiesRequest {
    fn default() -> Self {
        GetPhysicsPropertiesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPhysicsPropertiesResponse {
    pub time_step: f64,
    pub pause: bool,
    pub max_update_rate: f64,
    pub gravity: crate::geometry_msgs::msg::Vector3,
    pub ode_config: crate::gazebo_msgs::msg::ODEPhysics,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetPhysicsPropertiesResponse {
    fn default() -> Self {
        GetPhysicsPropertiesResponse {
            time_step: 0.0,
            pause: false,
            max_update_rate: 0.0,
            gravity: crate::geometry_msgs::msg::Vector3::default(),
            ode_config: crate::gazebo_msgs::msg::ODEPhysics::default(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct GetPhysicsProperties;
