use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TexturedMarkerArray {
    pub markers: Vec<crate::marti_visualization_msgs::msg::TexturedMarker>,
}

impl Default for TexturedMarkerArray {
    fn default() -> Self {
        TexturedMarkerArray {
            markers: Vec::new(),
        }
    }
}
