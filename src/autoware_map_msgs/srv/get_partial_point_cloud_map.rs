use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPartialPointCloudMapRequest {
    pub area: crate::autoware_map_msgs::msg::AreaInfo,
}

impl Default for GetPartialPointCloudMapRequest {
    fn default() -> Self {
        GetPartialPointCloudMapRequest {
            area: crate::autoware_map_msgs::msg::AreaInfo::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPartialPointCloudMapResponse {
    pub header: crate::std_msgs::msg::Header,
    pub new_pointcloud_with_ids: Vec<crate::autoware_map_msgs::msg::PointCloudMapCellWithID>,
}

impl Default for GetPartialPointCloudMapResponse {
    fn default() -> Self {
        GetPartialPointCloudMapResponse {
            header: crate::std_msgs::msg::Header::default(),
            new_pointcloud_with_ids: Vec::new(),
        }
    }
}

pub struct GetPartialPointCloudMap;
