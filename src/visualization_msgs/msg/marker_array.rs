use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MarkerArray {
    pub markers: Vec<crate::visualization_msgs::msg::Marker>,
}

impl Default for MarkerArray {
    fn default() -> Self {
        MarkerArray {
            markers: Vec::new(),
        }
    }
}
