use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDispatchStatesRequest {
    pub task_ids: Vec<::std::string::String>,
}

impl Default for GetDispatchStatesRequest {
    fn default() -> Self {
        GetDispatchStatesRequest {
            task_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDispatchStatesResponse {
    pub success: bool,
    pub states: crate::rmf_task_msgs::msg::DispatchStates,
}

impl Default for GetDispatchStatesResponse {
    fn default() -> Self {
        GetDispatchStatesResponse {
            success: false,
            states: crate::rmf_task_msgs::msg::DispatchStates::default(),
        }
    }
}

pub struct GetDispatchStates;
