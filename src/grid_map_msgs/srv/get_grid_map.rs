use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGridMapRequest {
    pub frame_id: ::std::string::String,
    pub position_x: f64,
    pub position_y: f64,
    pub length_x: f64,
    pub length_y: f64,
    pub layers: Vec<::std::string::String>,
}

impl Default for GetGridMapRequest {
    fn default() -> Self {
        GetGridMapRequest {
            frame_id: ::std::string::String::new(),
            position_x: 0.0,
            position_y: 0.0,
            length_x: 0.0,
            length_y: 0.0,
            layers: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGridMapResponse {
    pub map: crate::grid_map_msgs::msg::GridMap,
}

impl Default for GetGridMapResponse {
    fn default() -> Self {
        GetGridMapResponse {
            map: crate::grid_map_msgs::msg::GridMap::default(),
        }
    }
}

pub struct GetGridMap;
