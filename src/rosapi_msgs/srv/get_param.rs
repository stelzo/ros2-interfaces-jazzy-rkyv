use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetParamRequest {
    pub name: ::std::string::String,
    pub default_value: ::std::string::String,
}

impl Default for GetParamRequest {
    fn default() -> Self {
        GetParamRequest {
            name: ::std::string::String::new(),
            default_value: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetParamResponse {
    pub value: ::std::string::String,
}

impl Default for GetParamResponse {
    fn default() -> Self {
        GetParamResponse {
            value: ::std::string::String::new(),
        }
    }
}

pub struct GetParam;
