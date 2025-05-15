use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Angle {
    pub x: f32,
    pub y: f32,
}

impl Default for Angle {
    fn default() -> Self {
        Angle { x: 0.0, y: 0.0 }
    }
}
