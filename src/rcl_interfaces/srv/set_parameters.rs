use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetParametersRequest {
    pub parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for SetParametersRequest {
    fn default() -> Self {
        SetParametersRequest {
            parameters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetParametersResponse {
    pub results: Vec<crate::rcl_interfaces::msg::SetParametersResult>,
}

impl Default for SetParametersResponse {
    fn default() -> Self {
        SetParametersResponse {
            results: Vec::new(),
        }
    }
}

pub struct SetParameters;
