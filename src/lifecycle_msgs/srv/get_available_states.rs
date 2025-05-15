use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetAvailableStatesRequest {}

impl Default for GetAvailableStatesRequest {
    fn default() -> Self {
        GetAvailableStatesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetAvailableStatesResponse {
    pub available_states: Vec<crate::lifecycle_msgs::msg::State>,
}

impl Default for GetAvailableStatesResponse {
    fn default() -> Self {
        GetAvailableStatesResponse {
            available_states: Vec::new(),
        }
    }
}

pub struct GetAvailableStates;
