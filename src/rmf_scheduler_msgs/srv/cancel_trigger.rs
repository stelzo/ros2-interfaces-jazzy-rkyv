use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CancelTriggerRequest {
    pub name: ::std::string::String,
}

impl Default for CancelTriggerRequest {
    fn default() -> Self {
        CancelTriggerRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CancelTriggerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelTriggerResponse {
    fn default() -> Self {
        CancelTriggerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct CancelTrigger;
