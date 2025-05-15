use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TriggerRequest {}

impl Default for TriggerRequest {
    fn default() -> Self {
        TriggerRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TriggerResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for TriggerResponse {
    fn default() -> Self {
        TriggerResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct Trigger;
