use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UpdatePointcloudOctomapRequest {
    pub cloud: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for UpdatePointcloudOctomapRequest {
    fn default() -> Self {
        UpdatePointcloudOctomapRequest {
            cloud: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UpdatePointcloudOctomapResponse {
    pub success: bool,
}

impl Default for UpdatePointcloudOctomapResponse {
    fn default() -> Self {
        UpdatePointcloudOctomapResponse { success: false }
    }
}

pub struct UpdatePointcloudOctomap;
