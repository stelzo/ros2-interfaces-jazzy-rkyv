use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadMapRequest {
    pub map_url: ::std::string::String,
}

impl Default for LoadMapRequest {
    fn default() -> Self {
        LoadMapRequest {
            map_url: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadMapResponse {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub result: u8,
}

impl LoadMapResponse {
    pub const RESULT_SUCCESS: u8 = 0;
    pub const RESULT_MAP_DOES_NOT_EXIST: u8 = 1;
    pub const RESULT_INVALID_MAP_DATA: u8 = 2;
    pub const RESULT_INVALID_MAP_METADATA: u8 = 3;
    pub const RESULT_UNDEFINED_FAILURE: u8 = 255;
}

impl Default for LoadMapResponse {
    fn default() -> Self {
        LoadMapResponse {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
            result: 0,
        }
    }
}

pub struct LoadMap;
