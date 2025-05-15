use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DemuxDeleteRequest {
    pub topic: ::std::string::String,
}

impl Default for DemuxDeleteRequest {
    fn default() -> Self {
        DemuxDeleteRequest {
            topic: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DemuxDeleteResponse {
    pub success: bool,
}

impl Default for DemuxDeleteResponse {
    fn default() -> Self {
        DemuxDeleteResponse { success: false }
    }
}

pub struct DemuxDelete;
