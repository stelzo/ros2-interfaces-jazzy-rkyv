use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGroupUrdfRequest {
    pub group_name: ::std::string::String,
}

impl Default for GetGroupUrdfRequest {
    fn default() -> Self {
        GetGroupUrdfRequest {
            group_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGroupUrdfResponse {
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
    pub urdf_string: ::std::string::String,
}

impl Default for GetGroupUrdfResponse {
    fn default() -> Self {
        GetGroupUrdfResponse {
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
            urdf_string: ::std::string::String::new(),
        }
    }
}

pub struct GetGroupUrdf;
