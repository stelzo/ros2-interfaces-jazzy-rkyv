use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamSetRequest {
    pub param_id: ::std::string::String,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamSetRequest {
    fn default() -> Self {
        ParamSetRequest {
            param_id: ::std::string::String::new(),
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamSetResponse {
    pub success: bool,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamSetResponse {
    fn default() -> Self {
        ParamSetResponse {
            success: false,
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

pub struct ParamSet;
