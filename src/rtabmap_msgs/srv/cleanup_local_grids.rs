use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CleanupLocalGridsRequest {
    pub radius: i32,
    pub filter_scans: bool,
}

impl Default for CleanupLocalGridsRequest {
    fn default() -> Self {
        CleanupLocalGridsRequest {
            radius: 0,
            filter_scans: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CleanupLocalGridsResponse {
    pub modified: i32,
}

impl Default for CleanupLocalGridsResponse {
    fn default() -> Self {
        CleanupLocalGridsResponse { modified: 0 }
    }
}

pub struct CleanupLocalGrids;
