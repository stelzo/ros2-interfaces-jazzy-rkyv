use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLoadCarriersRequest {
    pub load_carrier_ids: Vec<::std::string::String>,
}

impl Default for GetLoadCarriersRequest {
    fn default() -> Self {
        GetLoadCarriersRequest {
            load_carrier_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLoadCarriersResponse {
    pub load_carriers: Vec<crate::rc_reason_msgs::msg::LoadCarrierModel>,
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for GetLoadCarriersResponse {
    fn default() -> Self {
        GetLoadCarriersResponse {
            load_carriers: Vec::new(),
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct GetLoadCarriers;
