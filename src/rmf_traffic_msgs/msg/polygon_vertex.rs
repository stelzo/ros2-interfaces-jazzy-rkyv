use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PolygonVertex {
    pub x: f32,
    pub y: f32,
}

impl Default for PolygonVertex {
    fn default() -> Self {
        PolygonVertex { x: 0.0, y: 0.0 }
    }
}
