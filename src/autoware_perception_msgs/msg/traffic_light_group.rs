use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TrafficLightGroup {
    pub traffic_light_group_id: i64,
    pub elements: Vec<crate::autoware_perception_msgs::msg::TrafficLightElement>,
}

impl Default for TrafficLightGroup {
    fn default() -> Self {
        TrafficLightGroup {
            traffic_light_group_id: 0,
            elements: Vec::new(),
        }
    }
}
