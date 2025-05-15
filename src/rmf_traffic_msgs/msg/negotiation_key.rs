use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NegotiationKey {
    pub participant: u64,
    pub version: u64,
}

impl Default for NegotiationKey {
    fn default() -> Self {
        NegotiationKey {
            participant: 0,
            version: 0,
        }
    }
}
