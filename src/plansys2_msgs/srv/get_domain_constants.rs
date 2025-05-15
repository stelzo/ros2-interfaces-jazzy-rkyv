use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainConstantsRequest {
    pub r#type: ::std::string::String,
}

impl Default for GetDomainConstantsRequest {
    fn default() -> Self {
        GetDomainConstantsRequest {
            r#type: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainConstantsResponse {
    pub success: bool,
    pub constants: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainConstantsResponse {
    fn default() -> Self {
        GetDomainConstantsResponse {
            success: false,
            constants: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetDomainConstants;
