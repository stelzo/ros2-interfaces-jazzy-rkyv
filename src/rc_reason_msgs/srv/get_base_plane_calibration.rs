use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetBasePlaneCalibrationRequest {
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for GetBasePlaneCalibrationRequest {
    fn default() -> Self {
        GetBasePlaneCalibrationRequest {
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetBasePlaneCalibrationResponse {
    pub pose_frame: ::std::string::String,
    pub plane: crate::shape_msgs::msg::Plane,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetBasePlaneCalibrationResponse {
    fn default() -> Self {
        GetBasePlaneCalibrationResponse {
            pose_frame: ::std::string::String::new(),
            plane: crate::shape_msgs::msg::Plane::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct GetBasePlaneCalibration;
