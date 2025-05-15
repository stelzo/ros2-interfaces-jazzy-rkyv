use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetModelStateRequest {
    pub model_state: crate::gazebo_msgs::msg::ModelState,
}

impl Default for SetModelStateRequest {
    fn default() -> Self {
        SetModelStateRequest {
            model_state: crate::gazebo_msgs::msg::ModelState::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetModelStateResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetModelStateResponse {
    fn default() -> Self {
        SetModelStateResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetModelState;
