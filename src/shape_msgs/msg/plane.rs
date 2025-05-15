use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Plane {
    pub coef: [f64; 4],
}

impl Default for Plane {
    fn default() -> Self {
        Plane { coef: [0.0; 4] }
    }
}
