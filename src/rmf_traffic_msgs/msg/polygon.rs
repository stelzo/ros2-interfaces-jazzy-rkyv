use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Polygon {
    pub vertices: Vec<crate::rmf_traffic_msgs::msg::PolygonVertex>,
}

impl Default for Polygon {
    fn default() -> Self {
        Polygon {
            vertices: Vec::new(),
        }
    }
}
