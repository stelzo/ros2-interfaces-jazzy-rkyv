use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPointmapLayerRequest {
    pub layer_name: ::std::string::String,
}

impl Default for GetPointmapLayerRequest {
    fn default() -> Self {
        GetPointmapLayerRequest {
            layer_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPointmapLayerResponse {
    pub valid: bool,
    pub points: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointmapLayerResponse {
    fn default() -> Self {
        GetPointmapLayerResponse {
            valid: false,
            points: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

pub struct GetPointmapLayer;
