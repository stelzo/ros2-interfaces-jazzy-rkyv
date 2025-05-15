use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CloseBlackboardStreamRequest {
    pub topic_name: ::std::string::String,
}

impl Default for CloseBlackboardStreamRequest {
    fn default() -> Self {
        CloseBlackboardStreamRequest {
            topic_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CloseBlackboardStreamResponse {
    pub result: bool,
}

impl Default for CloseBlackboardStreamResponse {
    fn default() -> Self {
        CloseBlackboardStreamResponse { result: false }
    }
}

pub struct CloseBlackboardStream;
