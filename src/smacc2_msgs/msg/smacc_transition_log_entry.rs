use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SmaccTransitionLogEntry {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub transition: crate::smacc2_msgs::msg::SmaccTransition,
}

impl Default for SmaccTransitionLogEntry {
    fn default() -> Self {
        SmaccTransitionLogEntry {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            transition: crate::smacc2_msgs::msg::SmaccTransition::default(),
        }
    }
}
