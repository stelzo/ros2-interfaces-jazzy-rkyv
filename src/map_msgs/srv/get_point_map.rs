use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPointMapRequest {}

impl Default for GetPointMapRequest {
    fn default() -> Self {
        GetPointMapRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPointMapResponse {
    pub map: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointMapResponse {
    fn default() -> Self {
        GetPointMapResponse {
            map: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

pub struct GetPointMap;
