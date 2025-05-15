use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WriteStateRequest {
    pub filename: ::std::string::String,
    pub include_unfinished_submaps: bool,
}

impl Default for WriteStateRequest {
    fn default() -> Self {
        WriteStateRequest {
            filename: ::std::string::String::new(),
            include_unfinished_submaps: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WriteStateResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
}

impl Default for WriteStateResponse {
    fn default() -> Self {
        WriteStateResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
        }
    }
}

pub struct WriteState;
