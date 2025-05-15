use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NegotiationStates {
    pub negotiations: Vec<crate::rmf_traffic_msgs::msg::NegotiationState>,
}

impl Default for NegotiationStates {
    fn default() -> Self {
        NegotiationStates {
            negotiations: Vec::new(),
        }
    }
}
