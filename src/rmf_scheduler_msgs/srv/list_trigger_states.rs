use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListTriggerStatesRequest {
    pub modified_after: i64,
}

impl Default for ListTriggerStatesRequest {
    fn default() -> Self {
        ListTriggerStatesRequest { modified_after: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListTriggerStatesResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub triggers: Vec<crate::rmf_scheduler_msgs::msg::TriggerState>,
}

impl Default for ListTriggerStatesResponse {
    fn default() -> Self {
        ListTriggerStatesResponse {
            success: false,
            message: ::std::string::String::new(),
            triggers: Vec::new(),
        }
    }
}

pub struct ListTriggerStates;
