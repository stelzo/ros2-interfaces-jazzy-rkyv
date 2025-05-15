use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PoseWithCovariance {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub covariance: [f64; 36],
}

impl Default for PoseWithCovariance {
    fn default() -> Self {
        PoseWithCovariance {
            pose: crate::geometry_msgs::msg::Pose::default(),
            covariance: [0.0; 36],
        }
    }
}
