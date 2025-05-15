use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Ptz {
    pub pan: f32,
    pub tilt: f32,
    pub zoom: f32,
}

impl Default for Ptz {
    fn default() -> Self {
        Ptz {
            pan: 0.0,
            tilt: 0.0,
            zoom: 0.0,
        }
    }
}
