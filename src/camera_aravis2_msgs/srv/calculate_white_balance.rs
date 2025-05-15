use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CalculateWhiteBalanceRequest {}

impl Default for CalculateWhiteBalanceRequest {
    fn default() -> Self {
        CalculateWhiteBalanceRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CalculateWhiteBalanceResponse {
    pub is_successful: bool,
    pub balance_ratios: Vec<crate::diagnostic_msgs::msg::KeyValue>,
}

impl Default for CalculateWhiteBalanceResponse {
    fn default() -> Self {
        CalculateWhiteBalanceResponse {
            is_successful: false,
            balance_ratios: Vec::new(),
        }
    }
}

pub struct CalculateWhiteBalance;
