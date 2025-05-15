use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteLoadCarriersRequest {
    pub load_carrier_ids: Vec<::std::string::String>,
}

impl Default for DeleteLoadCarriersRequest {
    fn default() -> Self {
        DeleteLoadCarriersRequest {
            load_carrier_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteLoadCarriersResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteLoadCarriersResponse {
    fn default() -> Self {
        DeleteLoadCarriersResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct DeleteLoadCarriers;
