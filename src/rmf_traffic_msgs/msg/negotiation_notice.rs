use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NegotiationNotice {
    pub conflict_version: u64,
    pub participants: Vec<u64>,
}

impl Default for NegotiationNotice {
    fn default() -> Self {
        NegotiationNotice {
            conflict_version: 0,
            participants: Vec::new(),
        }
    }
}
