use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamSetV2Request {
    pub force_set: bool,
    pub param_id: ::std::string::String,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
}

impl Default for ParamSetV2Request {
    fn default() -> Self {
        ParamSetV2Request {
            force_set: false,
            param_id: ::std::string::String::new(),
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamSetV2Response {
    pub success: bool,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
}

impl Default for ParamSetV2Response {
    fn default() -> Self {
        ParamSetV2Response {
            success: false,
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
        }
    }
}

pub struct ParamSetV2;
