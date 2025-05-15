use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApiServiceRequest {
    pub json_msg: ::std::string::String,
}

impl Default for ApiServiceRequest {
    fn default() -> Self {
        ApiServiceRequest {
            json_msg: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApiServiceResponse {
    pub json_msg: ::std::string::String,
}

impl Default for ApiServiceResponse {
    fn default() -> Self {
        ApiServiceResponse {
            json_msg: ::std::string::String::new(),
        }
    }
}

pub struct ApiService;
