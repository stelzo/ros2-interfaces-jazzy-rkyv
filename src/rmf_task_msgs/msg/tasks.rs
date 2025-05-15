use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Tasks {
    pub tasks: Vec<crate::rmf_task_msgs::msg::TaskSummary>,
}

impl Default for Tasks {
    fn default() -> Self {
        Tasks { tasks: Vec::new() }
    }
}
