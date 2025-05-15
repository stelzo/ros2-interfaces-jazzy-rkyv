use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CreateTriggerRequest {
    pub trigger: crate::rmf_scheduler_msgs::msg::Trigger,
}

impl Default for CreateTriggerRequest {
    fn default() -> Self {
        CreateTriggerRequest {
            trigger: crate::rmf_scheduler_msgs::msg::Trigger::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CreateTriggerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CreateTriggerResponse {
    fn default() -> Self {
        CreateTriggerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct CreateTrigger;
