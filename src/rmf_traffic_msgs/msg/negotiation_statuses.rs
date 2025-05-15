use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NegotiationStatuses {
    pub negotiations: Vec<crate::rmf_traffic_msgs::msg::NegotiationStatus>,
}

impl Default for NegotiationStatuses {
    fn default() -> Self {
        NegotiationStatuses {
            negotiations: Vec::new(),
        }
    }
}
