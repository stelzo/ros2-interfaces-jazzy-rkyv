use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetModelStateRequest {
    pub model_name: ::std::string::String,
    pub relative_entity_name: ::std::string::String,
}

impl Default for GetModelStateRequest {
    fn default() -> Self {
        GetModelStateRequest {
            model_name: ::std::string::String::new(),
            relative_entity_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetModelStateResponse {
    pub header: crate::std_msgs::msg::Header,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetModelStateResponse {
    fn default() -> Self {
        GetModelStateResponse {
            header: crate::std_msgs::msg::Header::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct GetModelState;
