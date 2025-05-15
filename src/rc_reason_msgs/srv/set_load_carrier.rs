use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLoadCarrierRequest {
    pub load_carrier: crate::rc_reason_msgs::msg::LoadCarrierModel,
}

impl Default for SetLoadCarrierRequest {
    fn default() -> Self {
        SetLoadCarrierRequest {
            load_carrier: crate::rc_reason_msgs::msg::LoadCarrierModel::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLoadCarrierResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for SetLoadCarrierResponse {
    fn default() -> Self {
        SetLoadCarrierResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct SetLoadCarrier;
