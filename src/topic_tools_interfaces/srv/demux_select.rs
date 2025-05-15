use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DemuxSelectRequest {
    pub topic: ::std::string::String,
}

impl Default for DemuxSelectRequest {
    fn default() -> Self {
        DemuxSelectRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DemuxSelectResponse {
    pub prev_topic: ::std::string::String,
    pub success: bool,
}

impl Default for DemuxSelectResponse {
    fn default() -> Self {
        DemuxSelectResponse {
            prev_topic: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct DemuxSelect;
