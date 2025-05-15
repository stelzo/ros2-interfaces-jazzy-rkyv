use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGridMapInfoRequest {}

impl Default for GetGridMapInfoRequest {
    fn default() -> Self {
        GetGridMapInfoRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetGridMapInfoResponse {
    pub info: crate::grid_map_msgs::msg::GridMapInfo,
}

impl Default for GetGridMapInfoResponse {
    fn default() -> Self {
        GetGridMapInfoResponse {
            info: crate::grid_map_msgs::msg::GridMapInfo::default(),
        }
    }
}

pub struct GetGridMapInfo;
