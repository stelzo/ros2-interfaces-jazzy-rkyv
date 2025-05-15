use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UpdateRouteMetadataRequest {
    pub route_guid: ::std::string::String,
    pub metadata_points: Vec<crate::marti_nav_msgs::msg::RoutePoint>,
}

impl Default for UpdateRouteMetadataRequest {
    fn default() -> Self {
        UpdateRouteMetadataRequest {
            route_guid: ::std::string::String::new(),
            metadata_points: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UpdateRouteMetadataResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for UpdateRouteMetadataResponse {
    fn default() -> Self {
        UpdateRouteMetadataResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct UpdateRouteMetadata;
