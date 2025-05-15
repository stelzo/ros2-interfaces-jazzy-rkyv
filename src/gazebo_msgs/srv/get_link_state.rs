use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLinkStateRequest {
    pub link_name: ::std::string::String,
    pub reference_frame: ::std::string::String,
}

impl Default for GetLinkStateRequest {
    fn default() -> Self {
        GetLinkStateRequest {
            link_name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLinkStateResponse {
    pub link_state: crate::gazebo_msgs::msg::LinkState,
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for GetLinkStateResponse {
    fn default() -> Self {
        GetLinkStateResponse {
            link_state: crate::gazebo_msgs::msg::LinkState::default(),
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct GetLinkState;
