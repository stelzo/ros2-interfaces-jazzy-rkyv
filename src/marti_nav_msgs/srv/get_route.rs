use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRouteRequest {
    pub guid: ::std::string::String,
}

impl Default for GetRouteRequest {
    fn default() -> Self {
        GetRouteRequest {
            guid: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetRouteResponse {
    pub route: crate::marti_nav_msgs::msg::Route,
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for GetRouteResponse {
    fn default() -> Self {
        GetRouteResponse {
            route: crate::marti_nav_msgs::msg::Route::default(),
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct GetRoute;
