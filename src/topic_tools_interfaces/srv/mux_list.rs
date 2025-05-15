use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MuxListRequest {}

impl Default for MuxListRequest {
    fn default() -> Self {
        MuxListRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MuxListResponse {
    pub topics: Vec<::std::string::String>,
}

impl Default for MuxListResponse {
    fn default() -> Self {
        MuxListResponse { topics: Vec::new() }
    }
}

pub struct MuxList;
