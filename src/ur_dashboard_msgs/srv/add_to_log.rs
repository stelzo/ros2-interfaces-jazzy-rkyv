use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddToLogRequest {
    pub message: ::std::string::String,
}

impl Default for AddToLogRequest {
    fn default() -> Self {
        AddToLogRequest {
            message: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddToLogResponse {
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for AddToLogResponse {
    fn default() -> Self {
        AddToLogResponse {
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct AddToLog;
