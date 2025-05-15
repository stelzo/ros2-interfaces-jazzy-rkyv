use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Space {
    pub shape: crate::rmf_traffic_msgs::msg::Shape,
    pub pose: crate::geometry_msgs::msg::Pose2D,
}

impl Default for Space {
    fn default() -> Self {
        Space {
            shape: crate::rmf_traffic_msgs::msg::Shape::default(),
            pose: crate::geometry_msgs::msg::Pose2D::default(),
        }
    }
}
