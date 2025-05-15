use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainRequest {
    fn default() -> Self {
        GetDomainRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainResponse {
    pub success: bool,
    pub domain: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainResponse {
    fn default() -> Self {
        GetDomainResponse {
            success: false,
            domain: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetDomain;
