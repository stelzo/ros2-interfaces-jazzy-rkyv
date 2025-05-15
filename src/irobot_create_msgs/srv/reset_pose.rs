use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ResetPoseRequest {
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for ResetPoseRequest {
    fn default() -> Self {
        ResetPoseRequest {
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ResetPoseResponse {}

impl Default for ResetPoseResponse {
    fn default() -> Self {
        ResetPoseResponse {}
    }
}

pub struct ResetPose;
