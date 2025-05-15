use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ModeEvent {
    pub timestamp: u64,
    pub start_mode: crate::system_modes_msgs::msg::Mode,
    pub goal_mode: crate::system_modes_msgs::msg::Mode,
}

impl Default for ModeEvent {
    fn default() -> Self {
        ModeEvent {
            timestamp: 0,
            start_mode: crate::system_modes_msgs::msg::Mode::default(),
            goal_mode: crate::system_modes_msgs::msg::Mode::default(),
        }
    }
}
