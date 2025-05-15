use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Obstacle {
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
    pub twist: crate::geometry_msgs::msg::TwistWithCovariance,
}

impl Default for Obstacle {
    fn default() -> Self {
        Obstacle {
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
            twist: crate::geometry_msgs::msg::TwistWithCovariance::default(),
        }
    }
}
