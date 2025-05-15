use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OpenBlackboardStreamRequest {
    pub variables: Vec<::std::string::String>,
    pub filter_on_visited_path: bool,
    pub with_activity_stream: bool,
}

impl Default for OpenBlackboardStreamRequest {
    fn default() -> Self {
        OpenBlackboardStreamRequest {
            variables: Vec::new(),
            filter_on_visited_path: false,
            with_activity_stream: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct OpenBlackboardStreamResponse {
    pub topic: ::std::string::String,
}

impl Default for OpenBlackboardStreamResponse {
    fn default() -> Self {
        OpenBlackboardStreamResponse {
            topic: ::std::string::String::new(),
        }
    }
}

pub struct OpenBlackboardStream;
