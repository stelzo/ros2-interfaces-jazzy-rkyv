use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetStateRequest {}

impl Default for GetStateRequest {
    fn default() -> Self {
        GetStateRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetStateResponse {
    pub current_state: crate::lifecycle_msgs::msg::State,
}

impl Default for GetStateResponse {
    fn default() -> Self {
        GetStateResponse {
            current_state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

pub struct GetState;
