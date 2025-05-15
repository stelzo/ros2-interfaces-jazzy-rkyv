use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SnapshotRequest {}

impl Default for SnapshotRequest {
    fn default() -> Self {
        SnapshotRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SnapshotResponse {
    pub success: bool,
}

impl Default for SnapshotResponse {
    fn default() -> Self {
        SnapshotResponse { success: false }
    }
}

pub struct Snapshot;
