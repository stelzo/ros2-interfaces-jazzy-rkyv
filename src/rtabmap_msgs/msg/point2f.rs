use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Point2f {
    pub x: f32,
    pub y: f32,
}

impl Default for Point2f {
    fn default() -> Self {
        Point2f { x: 0.0, y: 0.0 }
    }
}
