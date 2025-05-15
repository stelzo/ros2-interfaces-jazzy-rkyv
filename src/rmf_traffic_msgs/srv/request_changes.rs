use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RequestChangesRequest {
    pub query_id: u64,
    pub version: u64,
    pub full_update: bool, // default: false
}

impl Default for RequestChangesRequest {
    fn default() -> Self {
        RequestChangesRequest {
            query_id: 0,
            version: 0,
            full_update: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RequestChangesResponse {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub result: u8,
    pub error: ::std::string::String,
}

impl RequestChangesResponse {
    pub const REQUEST_ACCEPTED: u8 = 1;
    pub const UNKNOWN_QUERY_ID: u8 = 2;
    pub const ERROR: u8 = 3;
}

impl Default for RequestChangesResponse {
    fn default() -> Self {
        RequestChangesResponse {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            result: 0,
            error: ::std::string::String::new(),
        }
    }
}

pub struct RequestChanges;
