use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MuxSelectRequest {
    pub topic: ::std::string::String,
}

impl Default for MuxSelectRequest {
    fn default() -> Self {
        MuxSelectRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MuxSelectResponse {
    pub prev_topic: ::std::string::String,
    pub success: bool,
}

impl Default for MuxSelectResponse {
    fn default() -> Self {
        MuxSelectResponse {
            prev_topic: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct MuxSelect;
