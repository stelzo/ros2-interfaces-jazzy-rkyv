use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetParameterTypesRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for GetParameterTypesRequest {
    fn default() -> Self {
        GetParameterTypesRequest { names: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetParameterTypesResponse {
    pub types: Vec<u8>,
}

impl Default for GetParameterTypesResponse {
    fn default() -> Self {
        GetParameterTypesResponse { types: Vec::new() }
    }
}

pub struct GetParameterTypes;
