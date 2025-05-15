use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetParametersRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParametersRequest {
    fn default() -> Self {
        GetParametersRequest { names: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetParametersResponse {
    pub values: Vec<crate::rcl_interfaces::msg::ParameterValue>,
}

impl Default for GetParametersResponse {
    fn default() -> Self {
        GetParametersResponse { values: Vec::new() }
    }
}

pub struct GetParameters;
