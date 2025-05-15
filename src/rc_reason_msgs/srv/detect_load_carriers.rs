use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DetectLoadCarriersRequest {
    pub pose_frame: ::std::string::String,
    pub region_of_interest_id: ::std::string::String,
    pub region_of_interest_2d_id: ::std::string::String,
    pub load_carrier_ids: Vec<::std::string::String>,
    pub robot_pose: crate::geometry_msgs::msg::Pose,
}

impl Default for DetectLoadCarriersRequest {
    fn default() -> Self {
        DetectLoadCarriersRequest {
            pose_frame: ::std::string::String::new(),
            region_of_interest_id: ::std::string::String::new(),
            region_of_interest_2d_id: ::std::string::String::new(),
            load_carrier_ids: Vec::new(),
            robot_pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DetectLoadCarriersResponse {
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrier>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DetectLoadCarriersResponse {
    fn default() -> Self {
        DetectLoadCarriersResponse {
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct DetectLoadCarriers;
