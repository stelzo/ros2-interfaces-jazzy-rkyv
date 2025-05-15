use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PublishersRequest {
    pub topic: ::std::string::String,
}

impl Default for PublishersRequest {
    fn default() -> Self {
        PublishersRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PublishersResponse {
    pub publishers: Vec<::std::string::String>,
}

impl Default for PublishersResponse {
    fn default() -> Self {
        PublishersResponse {
            publishers: Vec::new(),
        }
    }
}

pub struct Publishers;
