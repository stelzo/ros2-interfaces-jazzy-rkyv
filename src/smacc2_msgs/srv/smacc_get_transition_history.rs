use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SmaccGetTransitionHistoryRequest {}

impl Default for SmaccGetTransitionHistoryRequest {
    fn default() -> Self {
        SmaccGetTransitionHistoryRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SmaccGetTransitionHistoryResponse {
    pub history: Vec<crate::smacc2_msgs::msg::SmaccTransitionLogEntry>,
}

impl Default for SmaccGetTransitionHistoryResponse {
    fn default() -> Self {
        SmaccGetTransitionHistoryResponse {
            history: Vec::new(),
        }
    }
}

pub struct SmaccGetTransitionHistory;
