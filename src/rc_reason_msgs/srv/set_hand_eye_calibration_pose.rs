use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetHandEyeCalibrationPoseRequest {
    pub slot: i32,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for SetHandEyeCalibrationPoseRequest {
    fn default() -> Self {
        SetHandEyeCalibrationPoseRequest {
            slot: 0,
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetHandEyeCalibrationPoseResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for SetHandEyeCalibrationPoseResponse {
    fn default() -> Self {
        SetHandEyeCalibrationPoseResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

pub struct SetHandEyeCalibrationPose;
