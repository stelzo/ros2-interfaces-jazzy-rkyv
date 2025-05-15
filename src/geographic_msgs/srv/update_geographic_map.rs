use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UpdateGeographicMapRequest {
    pub updates: crate::geographic_msgs::msg::GeographicMapChanges,
}

impl Default for UpdateGeographicMapRequest {
    fn default() -> Self {
        UpdateGeographicMapRequest {
            updates: crate::geographic_msgs::msg::GeographicMapChanges::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UpdateGeographicMapResponse {
    pub success: bool,
    pub status: ::std::string::String,
}

impl Default for UpdateGeographicMapResponse {
    fn default() -> Self {
        UpdateGeographicMapResponse {
            success: false,
            status: ::std::string::String::new(),
        }
    }
}

pub struct UpdateGeographicMap;
