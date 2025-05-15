use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetMapProjectionsRequest {}

impl Default for SetMapProjectionsRequest {
    fn default() -> Self {
        SetMapProjectionsRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetMapProjectionsResponse {
    pub projected_maps_info: Vec<crate::map_msgs::msg::ProjectedMapInfo>,
}

impl Default for SetMapProjectionsResponse {
    fn default() -> Self {
        SetMapProjectionsResponse {
            projected_maps_info: Vec::new(),
        }
    }
}

pub struct SetMapProjections;
