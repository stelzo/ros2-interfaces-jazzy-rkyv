use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPointMapROIRequest {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub r: f64,
    pub l_x: f64,
    pub l_y: f64,
    pub l_z: f64,
}

impl Default for GetPointMapROIRequest {
    fn default() -> Self {
        GetPointMapROIRequest {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            r: 0.0,
            l_x: 0.0,
            l_y: 0.0,
            l_z: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPointMapROIResponse {
    pub sub_map: crate::sensor_msgs::msg::PointCloud2,
}

impl Default for GetPointMapROIResponse {
    fn default() -> Self {
        GetPointMapROIResponse {
            sub_map: crate::sensor_msgs::msg::PointCloud2::default(),
        }
    }
}

pub struct GetPointMapROI;
