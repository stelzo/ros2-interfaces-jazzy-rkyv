use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MutexGroupStates {
    pub assignments: Vec<crate::rmf_fleet_msgs::msg::MutexGroupAssignment>,
}

impl Default for MutexGroupStates {
    fn default() -> Self {
        MutexGroupStates {
            assignments: Vec::new(),
        }
    }
}
