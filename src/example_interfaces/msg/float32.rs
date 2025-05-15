use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Float32 {
    pub data: f32,
}

impl Default for Float32 {
    fn default() -> Self {
        Float32 { data: 0.0 }
    }
}
