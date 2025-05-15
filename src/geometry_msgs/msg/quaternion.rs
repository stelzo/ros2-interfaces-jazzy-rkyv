use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Quaternion {
    pub x: f64, // default: 0
    pub y: f64, // default: 0
    pub z: f64, // default: 0
    pub w: f64, // default: 1
}

impl Default for Quaternion {
    fn default() -> Self {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}
