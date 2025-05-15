use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetGridMapRequest {
    pub map: crate::grid_map_msgs::msg::GridMap,
}

impl Default for SetGridMapRequest {
    fn default() -> Self {
        SetGridMapRequest {
            map: crate::grid_map_msgs::msg::GridMap::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetGridMapResponse {}

impl Default for SetGridMapResponse {
    fn default() -> Self {
        SetGridMapResponse {}
    }
}

pub struct SetGridMap;
