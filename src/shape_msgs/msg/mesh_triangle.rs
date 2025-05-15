use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MeshTriangle {
    pub vertex_indices: [u32; 3],
}

impl Default for MeshTriangle {
    fn default() -> Self {
        MeshTriangle {
            vertex_indices: [0; 3],
        }
    }
}
