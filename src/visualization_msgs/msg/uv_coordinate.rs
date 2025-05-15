use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UVCoordinate {
    pub u: f32,
    pub v: f32,
}

impl Default for UVCoordinate {
    fn default() -> Self {
        UVCoordinate { u: 0.0, v: 0.0 }
    }
}
