use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TrafficLightGroupArray {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub traffic_light_groups: Vec<crate::autoware_perception_msgs::msg::TrafficLightGroup>,
}

impl Default for TrafficLightGroupArray {
    fn default() -> Self {
        TrafficLightGroupArray {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            traffic_light_groups: Vec::new(),
        }
    }
}
