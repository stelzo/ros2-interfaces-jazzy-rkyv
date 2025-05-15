use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CadMatchDetectObjectRequest {
    pub template_id: ::std::string::String,
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub load_carrier_id: ::std::string::String,
    pub load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
    pub collision_detection: crate::rc_reason_msgs::msg::CollisionDetection,
    pub pose_prior_ids: Vec<::std::string::String>,
    pub data_acquisition_mode: ::std::string::String,
}

impl Default for CadMatchDetectObjectRequest {
    fn default() -> Self {
        CadMatchDetectObjectRequest {
            template_id: ::std::string::String::new(),
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            load_carrier_id: ::std::string::String::new(),
            load_carrier_compartment: crate::rc_reason_msgs::msg::Compartment::default(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
            collision_detection: crate::rc_reason_msgs::msg::CollisionDetection::default(),
            pose_prior_ids: Vec::new(),
            data_acquisition_mode: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CadMatchDetectObjectResponse {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub matches: Vec<crate::rc_reason_msgs::msg::Match>,
    pub grasps: Vec<crate::rc_reason_msgs::msg::Grasp>,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for CadMatchDetectObjectResponse {
    fn default() -> Self {
        CadMatchDetectObjectResponse {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            matches: Vec::new(),
            grasps: Vec::new(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct CadMatchDetectObject;
