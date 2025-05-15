use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Float32Array {
    pub data: Vec<f32>,
}

impl Default for Float32Array {
    fn default() -> Self {
        Float32Array { data: Vec::new() }
    }
}
