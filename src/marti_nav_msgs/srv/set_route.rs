use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetRouteRequest {
    pub guid: ::std::string::String,
    pub repeat: bool,
}

impl Default for SetRouteRequest {
    fn default() -> Self {
        SetRouteRequest {
            guid: ::std::string::String::new(),
            repeat: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetRouteResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetRouteResponse {
    fn default() -> Self {
        SetRouteResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct SetRoute;
