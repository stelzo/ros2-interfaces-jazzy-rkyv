use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetEntityStateRequest {
    pub name: ::std::string::String,
    pub reference_frame: ::std::string::String,
}

impl Default for GetEntityStateRequest {
    fn default() -> Self {
        GetEntityStateRequest {
            name: ::std::string::String::new(),
            reference_frame: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetEntityStateResponse {
    pub header: crate::std_msgs::msg::Header,
    pub state: crate::gazebo_msgs::msg::EntityState,
    pub success: bool,
}

impl Default for GetEntityStateResponse {
    fn default() -> Self {
        GetEntityStateResponse {
            header: crate::std_msgs::msg::Header::default(),
            state: crate::gazebo_msgs::msg::EntityState::default(),
            success: false,
        }
    }
}

pub struct GetEntityState;
