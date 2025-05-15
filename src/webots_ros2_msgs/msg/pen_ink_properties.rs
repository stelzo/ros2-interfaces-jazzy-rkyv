use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PenInkProperties {
    pub color: i32,
    pub density: f32,
}

impl Default for PenInkProperties {
    fn default() -> Self {
        PenInkProperties {
            color: 0,
            density: 0.0,
        }
    }
}
