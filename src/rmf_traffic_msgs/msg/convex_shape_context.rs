use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ConvexShapeContext {
    pub circles: Vec<crate::rmf_traffic_msgs::msg::Circle>,
}

impl Default for ConvexShapeContext {
    fn default() -> Self {
        ConvexShapeContext {
            circles: Vec::new(),
        }
    }
}
