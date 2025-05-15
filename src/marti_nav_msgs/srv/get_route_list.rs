use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRouteListRequest {}

impl Default for GetRouteListRequest {
    fn default() -> Self {
        GetRouteListRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRouteListResponse {
    pub routes: Vec<crate::marti_nav_msgs::msg::Route>,
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetRouteListResponse {
    fn default() -> Self {
        GetRouteListResponse {
            routes: Vec::new(),
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct GetRouteList;
