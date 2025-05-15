use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetNodeDataRequest {
    pub ids: Vec<i32>,
    pub images: bool,
    pub scan: bool,
    pub grid: bool,
    pub user_data: bool,
}

impl Default for GetNodeDataRequest {
    fn default() -> Self {
        GetNodeDataRequest {
            ids: Vec::new(),
            images: false,
            scan: false,
            grid: false,
            user_data: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetNodeDataResponse {
    pub data: Vec<crate::rtabmap_msgs::msg::Node>,
}

impl Default for GetNodeDataResponse {
    fn default() -> Self {
        GetNodeDataResponse { data: Vec::new() }
    }
}

pub struct GetNodeData;
