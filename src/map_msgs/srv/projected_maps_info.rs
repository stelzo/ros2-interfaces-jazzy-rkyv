use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ProjectedMapsInfoRequest {
    pub projected_maps_info: Vec<crate::map_msgs::msg::ProjectedMapInfo>,
}

impl Default for ProjectedMapsInfoRequest {
    fn default() -> Self {
        ProjectedMapsInfoRequest {
            projected_maps_info: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ProjectedMapsInfoResponse {}

impl Default for ProjectedMapsInfoResponse {
    fn default() -> Self {
        ProjectedMapsInfoResponse {}
    }
}

pub struct ProjectedMapsInfo;
