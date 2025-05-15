use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NegotiationRepeat {
    pub conflict_version: u64,
    pub table: Vec<u64>,
}

impl Default for NegotiationRepeat {
    fn default() -> Self {
        NegotiationRepeat {
            conflict_version: 0,
            table: Vec::new(),
        }
    }
}
