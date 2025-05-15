use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Edges {
    pub node_ids: Vec<u32>,
    pub weights: Vec<f64>,
}

impl Default for Edges {
    fn default() -> Self {
        Edges {
            node_ids: Vec::new(),
            weights: Vec::new(),
        }
    }
}
