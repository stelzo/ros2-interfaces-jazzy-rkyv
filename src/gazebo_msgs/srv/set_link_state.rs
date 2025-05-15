use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLinkStateRequest {
    pub link_state: crate::gazebo_msgs::msg::LinkState,
}

impl Default for SetLinkStateRequest {
    fn default() -> Self {
        SetLinkStateRequest {
            link_state: crate::gazebo_msgs::msg::LinkState::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLinkStateResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetLinkStateResponse {
    fn default() -> Self {
        SetLinkStateResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetLinkState;
