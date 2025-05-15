use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGeoPathRequest {
    pub start: crate::geographic_msgs::msg::GeoPoint,
    pub goal: crate::geographic_msgs::msg::GeoPoint,
}

impl Default for GetGeoPathRequest {
    fn default() -> Self {
        GetGeoPathRequest {
            start: crate::geographic_msgs::msg::GeoPoint::default(),
            goal: crate::geographic_msgs::msg::GeoPoint::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGeoPathResponse {
    pub success: bool,
    pub status: ::std::string::String,
    pub plan: crate::geographic_msgs::msg::GeoPath,
    pub network: crate::unique_identifier_msgs::msg::UUID,
    pub start_seg: crate::unique_identifier_msgs::msg::UUID,
    pub goal_seg: crate::unique_identifier_msgs::msg::UUID,
    pub distance: f64,
}

impl Default for GetGeoPathResponse {
    fn default() -> Self {
        GetGeoPathResponse {
            success: false,
            status: ::std::string::String::new(),
            plan: crate::geographic_msgs::msg::GeoPath::default(),
            network: crate::unique_identifier_msgs::msg::UUID::default(),
            start_seg: crate::unique_identifier_msgs::msg::UUID::default(),
            goal_seg: crate::unique_identifier_msgs::msg::UUID::default(),
            distance: 0.0,
        }
    }
}

pub struct GetGeoPath;
