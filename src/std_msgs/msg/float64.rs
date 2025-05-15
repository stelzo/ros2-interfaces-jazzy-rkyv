use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Float64 {
    pub data: f64,
}

impl Default for Float64 {
    fn default() -> Self {
        Float64 { data: 0.0 }
    }
}
