use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NegotiationRefusal {
    pub conflict_version: u64,
}

impl Default for NegotiationRefusal {
    fn default() -> Self {
        NegotiationRefusal {
            conflict_version: 0,
        }
    }
}
