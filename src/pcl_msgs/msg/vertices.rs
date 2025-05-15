use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Vertices {
    pub vertices: Vec<u32>,
}

impl Default for Vertices {
    fn default() -> Self {
        Vertices {
            vertices: Vec::new(),
        }
    }
}
