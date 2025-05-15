use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGridmapLayerRequest {
    pub layer_name: ::std::string::String,
}

impl Default for GetGridmapLayerRequest {
    fn default() -> Self {
        GetGridmapLayerRequest {
            layer_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGridmapLayerResponse {
    pub valid: bool,
    pub grid: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetGridmapLayerResponse {
    fn default() -> Self {
        GetGridmapLayerResponse {
            valid: false,
            grid: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

pub struct GetGridmapLayer;
