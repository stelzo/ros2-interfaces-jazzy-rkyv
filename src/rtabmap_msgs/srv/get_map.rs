use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMapRequest {
    pub global_map: bool,
    pub optimized: bool,
    pub graph_only: bool,
}

impl Default for GetMapRequest {
    fn default() -> Self {
        GetMapRequest {
            global_map: false,
            optimized: false,
            graph_only: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMapResponse {
    pub data: crate::rtabmap_msgs::msg::MapData,
}

impl Default for GetMapResponse {
    fn default() -> Self {
        GetMapResponse {
            data: crate::rtabmap_msgs::msg::MapData::default(),
        }
    }
}

pub struct GetMap;
