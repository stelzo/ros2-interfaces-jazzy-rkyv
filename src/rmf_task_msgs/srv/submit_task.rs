use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SubmitTaskRequest {
    pub requester: ::std::string::String,
    pub description: crate::rmf_task_msgs::msg::TaskDescription,
}

impl Default for SubmitTaskRequest {
    fn default() -> Self {
        SubmitTaskRequest {
            requester: ::std::string::String::new(),
            description: crate::rmf_task_msgs::msg::TaskDescription::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SubmitTaskResponse {
    pub success: bool,
    pub task_id: ::std::string::String,
    pub message: ::std::string::String,
}

impl Default for SubmitTaskResponse {
    fn default() -> Self {
        SubmitTaskResponse {
            success: false,
            task_id: ::std::string::String::new(),
            message: ::std::string::String::new(),
        }
    }
}

pub struct SubmitTask;
