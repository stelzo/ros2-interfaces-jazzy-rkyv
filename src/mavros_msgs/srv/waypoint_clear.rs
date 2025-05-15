use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointClearRequest {}

impl Default for WaypointClearRequest {
    fn default() -> Self {
        WaypointClearRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointClearResponse {
    pub success: bool,
}

impl Default for WaypointClearResponse {
    fn default() -> Self {
        WaypointClearResponse { success: false }
    }
}

pub struct WaypointClear;
