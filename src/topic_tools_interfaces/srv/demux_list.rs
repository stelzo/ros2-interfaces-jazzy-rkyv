use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DemuxListRequest {}

impl Default for DemuxListRequest {
    fn default() -> Self {
        DemuxListRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DemuxListResponse {
    pub topics: Vec<::std::string::String>,
}

impl Default for DemuxListResponse {
    fn default() -> Self {
        DemuxListResponse { topics: Vec::new() }
    }
}

pub struct DemuxList;
