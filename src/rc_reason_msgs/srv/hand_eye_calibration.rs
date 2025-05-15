use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HandEyeCalibrationRequest {}

impl Default for HandEyeCalibrationRequest {
    fn default() -> Self {
        HandEyeCalibrationRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HandEyeCalibrationResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub error: f64,
    pub translation_error_meter: f64,
    pub rotation_error_degree: f64,
    pub robot_mounted: bool,
}

impl Default for HandEyeCalibrationResponse {
    fn default() -> Self {
        HandEyeCalibrationResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            error: 0.0,
            translation_error_meter: 0.0,
            rotation_error_degree: 0.0,
            robot_mounted: false,
        }
    }
}

pub struct HandEyeCalibration;
