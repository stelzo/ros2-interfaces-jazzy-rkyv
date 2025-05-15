use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveRouteRequest {
    pub name: ::std::string::String,
    pub guid: ::std::string::String,
    pub route: crate::marti_nav_msgs::msg::Route,
    pub thumbnail: ::std::string::String,
}

impl Default for SaveRouteRequest {
    fn default() -> Self {
        SaveRouteRequest {
            name: ::std::string::String::new(),
            guid: ::std::string::String::new(),
            route: crate::marti_nav_msgs::msg::Route::default(),
            thumbnail: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SaveRouteResponse {
    fn default() -> Self {
        SaveRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct SaveRoute;
