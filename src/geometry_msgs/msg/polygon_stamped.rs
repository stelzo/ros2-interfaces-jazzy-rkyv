use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PolygonStamped {
    pub header: crate::std_msgs::msg::Header,
    pub polygon: crate::geometry_msgs::msg::Polygon,
}

impl Default for PolygonStamped {
    fn default() -> Self {
        PolygonStamped {
            header: crate::std_msgs::msg::Header::default(),
            polygon: crate::geometry_msgs::msg::Polygon::default(),
        }
    }
}
