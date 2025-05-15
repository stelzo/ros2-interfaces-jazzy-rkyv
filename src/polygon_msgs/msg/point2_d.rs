use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Default for Point2D {
    fn default() -> Self {
        Point2D { x: 0.0, y: 0.0 }
    }
}
