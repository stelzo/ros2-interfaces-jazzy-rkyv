use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TrafficSignalArray {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub signals: Vec<crate::autoware_perception_msgs::msg::TrafficSignal>,
}

impl Default for TrafficSignalArray {
    fn default() -> Self {
        TrafficSignalArray {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            signals: Vec::new(),
        }
    }
}
