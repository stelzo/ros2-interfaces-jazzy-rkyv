use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NodeDetailsRequest {
    pub node: ::std::string::String,
}

impl Default for NodeDetailsRequest {
    fn default() -> Self {
        NodeDetailsRequest {
            node: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NodeDetailsResponse {
    pub subscribing: Vec<::std::string::String>,
    pub publishing: Vec<::std::string::String>,
    pub services: Vec<::std::string::String>,
}

impl Default for NodeDetailsResponse {
    fn default() -> Self {
        NodeDetailsResponse {
            subscribing: Vec::new(),
            publishing: Vec::new(),
            services: Vec::new(),
        }
    }
}

pub struct NodeDetails;
