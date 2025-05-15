use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ShapeContext {
    pub convex_shapes: crate::rmf_traffic_msgs::msg::ConvexShapeContext,
}

impl Default for ShapeContext {
    fn default() -> Self {
        ShapeContext {
            convex_shapes: crate::rmf_traffic_msgs::msg::ConvexShapeContext::default(),
        }
    }
}
