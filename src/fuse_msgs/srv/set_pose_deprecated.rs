use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPoseDeprecatedRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetPoseDeprecatedRequest {
    fn default() -> Self {
        SetPoseDeprecatedRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPoseDeprecatedResponse {}

impl Default for SetPoseDeprecatedResponse {
    fn default() -> Self {
        SetPoseDeprecatedResponse {}
    }
}

pub struct SetPoseDeprecated;
