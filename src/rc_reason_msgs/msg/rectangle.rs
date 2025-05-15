use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle { x: 0.0, y: 0.0 }
    }
}
