use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Polygon {
    pub points: Vec<crate::geometry_msgs::msg::Point32>,
}

impl Default for Polygon {
    fn default() -> Self {
        Polygon { points: Vec::new() }
    }
}
