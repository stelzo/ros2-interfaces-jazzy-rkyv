use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetEntityStateRequest {
    pub state: crate::gazebo_msgs::msg::EntityState,
}

impl Default for SetEntityStateRequest {
    fn default() -> Self {
        SetEntityStateRequest {
            state: crate::gazebo_msgs::msg::EntityState::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetEntityStateResponse {
    pub success: bool,
}

impl Default for SetEntityStateResponse {
    fn default() -> Self {
        SetEntityStateResponse { success: false }
    }
}

pub struct SetEntityState;
