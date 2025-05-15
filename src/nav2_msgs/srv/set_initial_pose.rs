use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetInitialPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetInitialPoseRequest {
    fn default() -> Self {
        SetInitialPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetInitialPoseResponse {}

impl Default for SetInitialPoseResponse {
    fn default() -> Self {
        SetInitialPoseResponse {}
    }
}

pub struct SetInitialPose;
