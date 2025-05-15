use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HazardStatusStamped {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub status: crate::autoware_system_msgs::msg::HazardStatus,
}

impl Default for HazardStatusStamped {
    fn default() -> Self {
        HazardStatusStamped {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            status: crate::autoware_system_msgs::msg::HazardStatus::default(),
        }
    }
}
