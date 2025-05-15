use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GoalInfo {
    pub goal_id: crate::unique_identifier_msgs::msg::UUID,
    pub stamp: crate::builtin_interfaces::msg::Time,
}

impl Default for GoalInfo {
    fn default() -> Self {
        GoalInfo {
            goal_id: crate::unique_identifier_msgs::msg::UUID::default(),
            stamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}
