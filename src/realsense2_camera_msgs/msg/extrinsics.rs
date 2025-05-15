use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Extrinsics {
    pub rotation: [f64; 9],
    pub translation: [f64; 3],
}

impl Default for Extrinsics {
    fn default() -> Self {
        Extrinsics {
            rotation: [0.0; 9],
            translation: [0.0; 3],
        }
    }
}
