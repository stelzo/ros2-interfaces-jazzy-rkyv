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
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for TriggerResponse {
    fn default() -> Self {
        TriggerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct Trigger;
