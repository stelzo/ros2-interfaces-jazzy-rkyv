use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetAvailableTransitionsRequest {}

impl Default for GetAvailableTransitionsRequest {
    fn default() -> Self {
        GetAvailableTransitionsRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetAvailableTransitionsResponse {
    pub available_transitions: Vec<crate::lifecycle_msgs::msg::TransitionDescription>,
}

impl Default for GetAvailableTransitionsResponse {
    fn default() -> Self {
        GetAvailableTransitionsResponse {
            available_transitions: Vec::new(),
        }
    }
}

pub struct GetAvailableTransitions;
