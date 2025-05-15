use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Point32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Point32 {
    fn default() -> Self {
        Point32 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
