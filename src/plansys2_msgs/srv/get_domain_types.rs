use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainTypesRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetDomainTypesRequest {
    fn default() -> Self {
        GetDomainTypesRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetDomainTypesResponse {
    pub success: bool,
    pub types: Vec<::std::string::String>,
    pub error_info: ::std::string::String,
}

impl Default for GetDomainTypesResponse {
    fn default() -> Self {
        GetDomainTypesResponse {
            success: false,
            types: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetDomainTypes;
