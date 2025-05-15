use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RelocalizeNearPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for RelocalizeNearPoseRequest {
    fn default() -> Self {
        RelocalizeNearPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RelocalizeNearPoseResponse {
    pub accepted: bool,
}

impl Default for RelocalizeNearPoseResponse {
    fn default() -> Self {
        RelocalizeNearPoseResponse { accepted: false }
    }
}

pub struct RelocalizeNearPose;
