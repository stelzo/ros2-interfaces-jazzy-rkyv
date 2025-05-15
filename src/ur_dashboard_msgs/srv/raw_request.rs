use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RawRequestRequest {
    pub query: ::std::string::String,
}

impl Default for RawRequestRequest {
    fn default() -> Self {
        RawRequestRequest {
            query: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RawRequestResponse {
    pub answer: ::std::string::String,
}

impl Default for RawRequestResponse {
    fn default() -> Self {
        RawRequestResponse {
            answer: ::std::string::String::new(),
        }
    }
}

pub struct RawRequest;
