use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NegotiationForfeit {
    pub conflict_version: u64,
    pub table: Vec<crate::rmf_traffic_msgs::msg::NegotiationKey>,
}

impl Default for NegotiationForfeit {
    fn default() -> Self {
        NegotiationForfeit {
            conflict_version: 0,
            table: Vec::new(),
        }
    }
}
