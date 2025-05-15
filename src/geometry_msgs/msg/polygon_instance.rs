use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PolygonInstance {
    pub polygon: crate::geometry_msgs::msg::Polygon,
    pub id: i64,
}

impl Default for PolygonInstance {
    fn default() -> Self {
        PolygonInstance {
            polygon: crate::geometry_msgs::msg::Polygon::default(),
            id: 0,
        }
    }
}
