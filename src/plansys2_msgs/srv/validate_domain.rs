use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ValidateDomainRequest {
    pub domain: ::std::string::String,
}

impl Default for ValidateDomainRequest {
    fn default() -> Self {
        ValidateDomainRequest {
            domain: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ValidateDomainResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for ValidateDomainResponse {
    fn default() -> Self {
        ValidateDomainResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct ValidateDomain;
