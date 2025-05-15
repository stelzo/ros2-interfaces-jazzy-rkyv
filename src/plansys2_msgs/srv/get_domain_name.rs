use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainNameRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainNameRequest {
    fn default() -> Self {
        GetDomainNameRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainNameResponse {
    pub success: bool,
    pub name: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainNameResponse {
    fn default() -> Self {
        GetDomainNameResponse {
            success: false,
            name: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetDomainName;
