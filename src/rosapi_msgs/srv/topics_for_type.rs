use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TopicsForTypeRequest {
    pub r#type: ::std::string::String,
}

impl Default for TopicsForTypeRequest {
    fn default() -> Self {
        TopicsForTypeRequest {
            r#type: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TopicsForTypeResponse {
    pub topics: Vec<::std::string::String>,
}

impl Default for TopicsForTypeResponse {
    fn default() -> Self {
        TopicsForTypeResponse { topics: Vec::new() }
    }
}

pub struct TopicsForType;
