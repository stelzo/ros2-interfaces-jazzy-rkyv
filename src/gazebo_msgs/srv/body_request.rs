use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BodyRequestRequest {
    pub body_name: ::std::string::String,
}

impl Default for BodyRequestRequest {
    fn default() -> Self {
        BodyRequestRequest {
            body_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BodyRequestResponse {}

impl Default for BodyRequestResponse {
    fn default() -> Self {
        BodyRequestResponse {}
    }
}

pub struct BodyRequest;
