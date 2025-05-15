use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteRouteRequest {
    pub guid: ::std::string::String,
}

impl Default for DeleteRouteRequest {
    fn default() -> Self {
        DeleteRouteRequest {
            guid: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for DeleteRouteResponse {
    fn default() -> Self {
        DeleteRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct DeleteRoute;
