use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LinkRequestRequest {
    pub link_name: ::std::string::String,
}

impl Default for LinkRequestRequest {
    fn default() -> Self {
        LinkRequestRequest {
            link_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LinkRequestResponse {}

impl Default for LinkRequestResponse {
    fn default() -> Self {
        LinkRequestResponse {}
    }
}

pub struct LinkRequest;
