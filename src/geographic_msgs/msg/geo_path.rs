use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GeoPath {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::geographic_msgs::msg::GeoPoseStamped>,
}

impl Default for GeoPath {
    fn default() -> Self {
        GeoPath {
            header: crate::std_msgs::msg::Header::default(),
            poses: Vec::new(),
        }
    }
}
