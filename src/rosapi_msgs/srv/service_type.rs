use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceTypeRequest {
    pub service: ::std::string::String,
}

impl Default for ServiceTypeRequest {
    fn default() -> Self {
        ServiceTypeRequest {
            service: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceTypeResponse {
    pub r#type: ::std::string::String,
}

impl Default for ServiceTypeResponse {
    fn default() -> Self {
        ServiceTypeResponse {
            r#type: ::std::string::String::new(),
        }
    }
}

pub struct ServiceType;
