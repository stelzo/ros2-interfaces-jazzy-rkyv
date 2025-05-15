use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetHandEyeCalibrationRequest {
    pub pose: crate::geometry_msgs::msg::Pose,
    pub robot_mounted: bool,
}

impl Default for SetHandEyeCalibrationRequest {
    fn default() -> Self {
        SetHandEyeCalibrationRequest {
            pose: crate::geometry_msgs::msg::Pose::default(),
            robot_mounted: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetHandEyeCalibrationResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for SetHandEyeCalibrationResponse {
    fn default() -> Self {
        SetHandEyeCalibrationResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

pub struct SetHandEyeCalibration;
