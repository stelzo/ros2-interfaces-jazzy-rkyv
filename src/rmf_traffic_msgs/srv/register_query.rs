use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RegisterQueryRequest {
    pub query: crate::rmf_traffic_msgs::msg::ScheduleQuery,
}

impl Default for RegisterQueryRequest {
    fn default() -> Self {
        RegisterQueryRequest {
            query: crate::rmf_traffic_msgs::msg::ScheduleQuery::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RegisterQueryResponse {
    pub node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity,
    pub query_id: u64,
    pub error: ::std::string::String,
}

impl Default for RegisterQueryResponse {
    fn default() -> Self {
        RegisterQueryResponse {
            node_id: crate::rmf_traffic_msgs::msg::ScheduleIdentity::default(),
            query_id: 0,
            error: ::std::string::String::new(),
        }
    }
}

pub struct RegisterQuery;
