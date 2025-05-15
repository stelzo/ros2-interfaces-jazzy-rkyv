use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CancelTaskRequest {
    pub requester: ::std::string::String,
    pub task_id: ::std::string::String,
}

impl Default for CancelTaskRequest {
    fn default() -> Self {
        CancelTaskRequest {
            requester: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CancelTaskResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelTaskResponse {
    fn default() -> Self {
        CancelTaskResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct CancelTask;
