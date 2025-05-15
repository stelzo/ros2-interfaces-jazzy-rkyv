use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MarkerPose {
    pub marker_id: u16,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for MarkerPose {
    fn default() -> Self {
        MarkerPose {
            marker_id: 0,
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}
