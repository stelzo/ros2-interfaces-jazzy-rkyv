use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReviveTaskRequest {
    pub requester: ::std::string::String,
    pub task_id: ::std::string::String,
}

impl Default for ReviveTaskRequest {
    fn default() -> Self {
        ReviveTaskRequest {
            requester: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReviveTaskResponse {
    pub success: bool,
}

impl Default for ReviveTaskResponse {
    fn default() -> Self {
        ReviveTaskResponse { success: false }
    }
}

pub struct ReviveTask;
