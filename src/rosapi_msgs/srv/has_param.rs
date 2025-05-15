use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HasParamRequest {
    pub name: ::std::string::String,
}

impl Default for HasParamRequest {
    fn default() -> Self {
        HasParamRequest {
            name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HasParamResponse {
    pub exists: bool,
}

impl Default for HasParamResponse {
    fn default() -> Self {
        HasParamResponse { exists: false }
    }
}

pub struct HasParam;
