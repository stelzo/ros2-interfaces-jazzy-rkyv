use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ImageMarkerArray {
    pub markers: Vec<crate::visualization_msgs::msg::ImageMarker>,
}

impl Default for ImageMarkerArray {
    fn default() -> Self {
        ImageMarkerArray {
            markers: Vec::new(),
        }
    }
}
