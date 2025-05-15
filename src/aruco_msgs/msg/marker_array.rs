use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MarkerArray {
    pub header: crate::std_msgs::msg::Header,
    pub markers: Vec<crate::aruco_msgs::msg::Marker>,
}

impl Default for MarkerArray {
    fn default() -> Self {
        MarkerArray {
            header: crate::std_msgs::msg::Header::default(),
            markers: Vec::new(),
        }
    }
}
