use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TopicTypeRequest {
    pub topic: ::std::string::String,
}

impl Default for TopicTypeRequest {
    fn default() -> Self {
        TopicTypeRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TopicTypeResponse {
    pub r#type: ::std::string::String,
}

impl Default for TopicTypeResponse {
    fn default() -> Self {
        TopicTypeResponse {
            r#type: ::std::string::String::new(),
        }
    }
}

pub struct TopicType;
