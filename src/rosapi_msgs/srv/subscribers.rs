use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SubscribersRequest {
    pub topic: ::std::string::String,
}

impl Default for SubscribersRequest {
    fn default() -> Self {
        SubscribersRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SubscribersResponse {
    pub subscribers: Vec<::std::string::String>,
}

impl Default for SubscribersResponse {
    fn default() -> Self {
        SubscribersResponse {
            subscribers: Vec::new(),
        }
    }
}

pub struct Subscribers;
