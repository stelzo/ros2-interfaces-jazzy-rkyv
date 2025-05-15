use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetSelectedPointCloudMapRequest {
    pub cell_ids: Vec<::std::string::String>,
}

impl Default for GetSelectedPointCloudMapRequest {
    fn default() -> Self {
        GetSelectedPointCloudMapRequest {
            cell_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetSelectedPointCloudMapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
}

impl Default for GetSelectedPointCloudMapResponse {
    fn default() -> Self {
        GetSelectedPointCloudMapResponse {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
        }
    }
}

pub struct GetSelectedPointCloudMap;
