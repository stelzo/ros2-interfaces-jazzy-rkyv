use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LogRequestListRequest {
    pub start: u16,
    pub end: u16,
}

impl Default for LogRequestListRequest {
    fn default() -> Self {
        LogRequestListRequest { start: 0, end: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LogRequestListResponse {
    pub success: bool,
}

impl Default for LogRequestListResponse {
    fn default() -> Self {
        LogRequestListResponse { success: false }
    }
}

pub struct LogRequestList;
