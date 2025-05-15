use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BehaviorTreeLog {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub event_log: Vec<crate::nav2_msgs::msg::BehaviorTreeStatusChange>,
}

impl Default for BehaviorTreeLog {
    fn default() -> Self {
        BehaviorTreeLog {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            event_log: Vec::new(),
        }
    }
}
