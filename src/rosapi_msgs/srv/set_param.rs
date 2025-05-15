use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetParamRequest {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for SetParamRequest {
    fn default() -> Self {
        SetParamRequest {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetParamResponse {}

impl Default for SetParamResponse {
    fn default() -> Self {
        SetParamResponse {}
    }
}

pub struct SetParam;
