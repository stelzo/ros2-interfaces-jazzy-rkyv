use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Point3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Point3f {
    fn default() -> Self {
        Point3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
