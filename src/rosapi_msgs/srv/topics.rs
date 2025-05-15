use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TopicsRequest {}

impl Default for TopicsRequest {
    fn default() -> Self {
        TopicsRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TopicsResponse {
    pub topics: Vec<::std::string::String>,
    pub types: Vec<::std::string::String>,
}

impl Default for TopicsResponse {
    fn default() -> Self {
        TopicsResponse {
            topics: Vec::new(),
            types: Vec::new(),
        }
    }
}

pub struct Topics;
