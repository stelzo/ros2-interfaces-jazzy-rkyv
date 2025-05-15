use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteParamRequest {
    pub name: ::std::string::String,
}

impl Default for DeleteParamRequest {
    fn default() -> Self {
        DeleteParamRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteParamResponse {}

impl Default for DeleteParamResponse {
    fn default() -> Self {
        DeleteParamResponse {}
    }
}

pub struct DeleteParam;
