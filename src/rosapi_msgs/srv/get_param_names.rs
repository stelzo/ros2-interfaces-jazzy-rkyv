use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetParamNamesRequest {}

impl Default for GetParamNamesRequest {
    fn default() -> Self {
        GetParamNamesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetParamNamesResponse {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParamNamesResponse {
    fn default() -> Self {
        GetParamNamesResponse { names: Vec::new() }
    }
}

pub struct GetParamNames;
