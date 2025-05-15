use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Polygon2DCollection {
    pub header: crate::std_msgs::msg::Header,
    pub polygons: Vec<crate::polygon_msgs::msg::Polygon2D>,
    pub colors: Vec<crate::std_msgs::msg::ColorRGBA>,
}

impl Default for Polygon2DCollection {
    fn default() -> Self {
        Polygon2DCollection {
            header: crate::std_msgs::msg::Header::default(),
            polygons: Vec::new(),
            colors: Vec::new(),
        }
    }
}
