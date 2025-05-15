use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SmaccStateMachine {
    pub states: Vec<crate::smacc2_msgs::msg::SmaccState>,
}

impl Default for SmaccStateMachine {
    fn default() -> Self {
        SmaccStateMachine { states: Vec::new() }
    }
}
