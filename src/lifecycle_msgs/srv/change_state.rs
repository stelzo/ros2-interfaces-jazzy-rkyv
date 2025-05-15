use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ChangeStateRequest {
    pub transition: crate::lifecycle_msgs::msg::Transition,
}

impl Default for ChangeStateRequest {
    fn default() -> Self {
        ChangeStateRequest {
            transition: crate::lifecycle_msgs::msg::Transition::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ChangeStateResponse {
    pub success: bool,
}

impl Default for ChangeStateResponse {
    fn default() -> Self {
        ChangeStateResponse { success: false }
    }
}

pub struct ChangeState;
