use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceProvidersRequest {
    pub service: ::std::string::String,
}

impl Default for ServiceProvidersRequest {
    fn default() -> Self {
        ServiceProvidersRequest {
            service: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceProvidersResponse {
    pub providers: Vec<::std::string::String>,
}

impl Default for ServiceProvidersResponse {
    fn default() -> Self {
        ServiceProvidersResponse {
            providers: Vec::new(),
        }
    }
}

pub struct ServiceProviders;
