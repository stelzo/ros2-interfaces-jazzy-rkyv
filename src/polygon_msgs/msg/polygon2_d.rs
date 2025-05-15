use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Polygon2D {
    pub points: Vec<crate::polygon_msgs::msg::Point2D>,
}

impl Default for Polygon2D {
    fn default() -> Self {
        Polygon2D { points: Vec::new() }
    }
}
