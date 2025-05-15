use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GoalStatusArray {
    pub status_list: Vec<crate::action_msgs::msg::GoalStatus>,
}

impl Default for GoalStatusArray {
    fn default() -> Self {
        GoalStatusArray {
            status_list: Vec::new(),
        }
    }
}
