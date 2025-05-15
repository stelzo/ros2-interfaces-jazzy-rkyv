use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPoseRequest {
    pub pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetPoseRequest {
    fn default() -> Self {
        SetPoseRequest {
            pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPoseResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetPoseResponse {
    fn default() -> Self {
        SetPoseResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct SetPose;
