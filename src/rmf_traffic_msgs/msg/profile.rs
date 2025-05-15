use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Profile {
    pub footprint: crate::rmf_traffic_msgs::msg::ConvexShape,
    pub vicinity: crate::rmf_traffic_msgs::msg::ConvexShape,
    pub shape_context: crate::rmf_traffic_msgs::msg::ConvexShapeContext,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            footprint: crate::rmf_traffic_msgs::msg::ConvexShape::default(),
            vicinity: crate::rmf_traffic_msgs::msg::ConvexShape::default(),
            shape_context: crate::rmf_traffic_msgs::msg::ConvexShapeContext::default(),
        }
    }
}
