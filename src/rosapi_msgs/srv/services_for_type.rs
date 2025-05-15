use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServicesForTypeRequest {
    pub r#type: ::std::string::String,
}

impl Default for ServicesForTypeRequest {
    fn default() -> Self {
        ServicesForTypeRequest {
            r#type: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServicesForTypeResponse {
    pub services: Vec<::std::string::String>,
}

impl Default for ServicesForTypeResponse {
    fn default() -> Self {
        ServicesForTypeResponse {
            services: Vec::new(),
        }
    }
}

pub struct ServicesForType;
