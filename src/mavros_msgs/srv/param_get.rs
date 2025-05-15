use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamGetRequest {
    pub param_id: ::std::string::String,
}

impl Default for ParamGetRequest {
    fn default() -> Self {
        ParamGetRequest {
            param_id: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ParamGetResponse {
    pub success: bool,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamGetResponse {
    fn default() -> Self {
        ParamGetResponse {
            success: false,
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

pub struct ParamGet;
