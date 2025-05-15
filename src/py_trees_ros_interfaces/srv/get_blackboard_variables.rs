use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetBlackboardVariablesRequest {}

impl Default for GetBlackboardVariablesRequest {
    fn default() -> Self {
        GetBlackboardVariablesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetBlackboardVariablesResponse {
    pub variables: Vec<::std::string::String>,
}

impl Default for GetBlackboardVariablesResponse {
    fn default() -> Self {
        GetBlackboardVariablesResponse {
            variables: Vec::new(),
        }
    }
}

pub struct GetBlackboardVariables;
