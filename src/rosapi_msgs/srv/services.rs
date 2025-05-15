use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServicesRequest {}

impl Default for ServicesRequest {
    fn default() -> Self {
        ServicesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServicesResponse {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesResponse {
    fn default() -> Self {
        ServicesResponse {
            services: Vec::new(),
        }
    }
}

pub struct Services;
