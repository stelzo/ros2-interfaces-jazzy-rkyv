use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Polygon2DStamped {
    pub header: crate::std_msgs::msg::Header,
    pub polygon: crate::polygon_msgs::msg::Polygon2D,
}

impl Default for Polygon2DStamped {
    fn default() -> Self {
        Polygon2DStamped {
            header: crate::std_msgs::msg::Header::default(),
            polygon: crate::polygon_msgs::msg::Polygon2D::default(),
        }
    }
}
