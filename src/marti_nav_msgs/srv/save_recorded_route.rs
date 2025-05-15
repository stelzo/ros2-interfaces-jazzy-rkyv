use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveRecordedRouteRequest {
    pub name: ::std::string::String,
    pub thumbnail: ::std::string::String,
}

impl Default for SaveRecordedRouteRequest {
    fn default() -> Self {
        SaveRecordedRouteRequest {
            name: ::std::string::String::new(),
            thumbnail: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SaveRecordedRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SaveRecordedRouteResponse {
    fn default() -> Self {
        SaveRecordedRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct SaveRecordedRoute;
