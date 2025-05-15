use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetParametersAtomicallyRequest {
    pub parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for SetParametersAtomicallyRequest {
    fn default() -> Self {
        SetParametersAtomicallyRequest {
            parameters: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetParametersAtomicallyResponse {
    pub result: crate::rcl_interfaces::msg::SetParametersResult,
}

impl Default for SetParametersAtomicallyResponse {
    fn default() -> Self {
        SetParametersAtomicallyResponse {
            result: crate::rcl_interfaces::msg::SetParametersResult::default(),
        }
    }
}

pub struct SetParametersAtomically;
