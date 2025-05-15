use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OrientedBoundingBox {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub extents: crate::geometry_msgs::msg::Point32,
}

impl Default for OrientedBoundingBox {
    fn default() -> Self {
        OrientedBoundingBox {
            pose: crate::geometry_msgs::msg::Pose::default(),
            extents: crate::geometry_msgs::msg::Point32::default(),
        }
    }
}
