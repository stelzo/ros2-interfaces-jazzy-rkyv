use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMapRequest {}

impl Default for GetMapRequest {
    fn default() -> Self {
        GetMapRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMapResponse {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetMapResponse {
    fn default() -> Self {
        GetMapResponse {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

pub struct GetMap;
