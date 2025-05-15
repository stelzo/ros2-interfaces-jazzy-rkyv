use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TaskProfile {
    pub task_id: ::std::string::String,
    pub submission_time: crate::builtin_interfaces::msg::Time,
    pub description: crate::rmf_task_msgs::msg::TaskDescription,
}

impl Default for TaskProfile {
    fn default() -> Self {
        TaskProfile {
            task_id: ::std::string::String::new(),
            submission_time: crate::builtin_interfaces::msg::Time::default(),
            description: crate::rmf_task_msgs::msg::TaskDescription::default(),
        }
    }
}
