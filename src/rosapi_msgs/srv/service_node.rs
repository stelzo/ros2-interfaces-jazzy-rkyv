use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceNodeRequest {
    pub service: ::std::string::String,
}

impl Default for ServiceNodeRequest {
    fn default() -> Self {
        ServiceNodeRequest {
            service: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceNodeResponse {
    pub node: ::std::string::String,
}

impl Default for ServiceNodeResponse {
    fn default() -> Self {
        ServiceNodeResponse {
            node: ::std::string::String::new(),
        }
    }
}

pub struct ServiceNode;
