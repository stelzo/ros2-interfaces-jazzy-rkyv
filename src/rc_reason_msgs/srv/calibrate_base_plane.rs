use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CalibrateBasePlaneRequest {
    pub pose_frame: ::std::string::String,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
    pub plane_estimation_method: ::std::string::String,
    pub stereo_plane_preference: ::std::string::String,
    pub region_of_interest_2d_id: ::std::string::String,
    pub offset: f64,
    pub plane: crate::shape_msgs::msg::Plane,
}

impl Default for CalibrateBasePlaneRequest {
    fn default() -> Self {
        CalibrateBasePlaneRequest {
            pose_frame: ::std::string::String::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
            plane_estimation_method: ::std::string::String::new(),
            stereo_plane_preference: ::std::string::String::new(),
            region_of_interest_2d_id: ::std::string::String::new(),
            offset: 0.0,
            plane: crate::shape_msgs::msg::Plane::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CalibrateBasePlaneResponse {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub pose_frame: ::std::string::String,
    pub plane: crate::shape_msgs::msg::Plane,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for CalibrateBasePlaneResponse {
    fn default() -> Self {
        CalibrateBasePlaneResponse {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            pose_frame: ::std::string::String::new(),
            plane: crate::shape_msgs::msg::Plane::default(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct CalibrateBasePlane;
