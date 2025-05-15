use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DemuxAddRequest {
    pub topic: ::std::string::String,
}

impl Default for DemuxAddRequest {
    fn default() -> Self {
        DemuxAddRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DemuxAddResponse {
    pub success: bool,
}

impl Default for DemuxAddResponse {
    fn default() -> Self {
        DemuxAddResponse { success: false }
    }
}

pub struct DemuxAdd;
