use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListTriggersRequest {
    pub created_after: i64,
}

impl Default for ListTriggersRequest {
    fn default() -> Self {
        ListTriggersRequest { created_after: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListTriggersResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub triggers: Vec<crate::rmf_scheduler_msgs::msg::Trigger>,
}

impl Default for ListTriggersResponse {
    fn default() -> Self {
        ListTriggersResponse {
            success: false,
            message: ::std::string::String::new(),
            triggers: Vec::new(),
        }
    }
}

pub struct ListTriggers;
