use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TransitionDescription {
    pub transition: crate::lifecycle_msgs::msg::Transition,
    pub start_state: crate::lifecycle_msgs::msg::State,
    pub goal_state: crate::lifecycle_msgs::msg::State,
}

impl Default for TransitionDescription {
    fn default() -> Self {
        TransitionDescription {
            transition: crate::lifecycle_msgs::msg::Transition::default(),
            start_state: crate::lifecycle_msgs::msg::State::default(),
            goal_state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}
