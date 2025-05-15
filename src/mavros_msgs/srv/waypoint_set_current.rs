use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointSetCurrentRequest {
    pub wp_seq: u16,
}

impl Default for WaypointSetCurrentRequest {
    fn default() -> Self {
        WaypointSetCurrentRequest { wp_seq: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointSetCurrentResponse {
    pub success: bool,
}

impl Default for WaypointSetCurrentResponse {
    fn default() -> Self {
        WaypointSetCurrentResponse { success: false }
    }
}

pub struct WaypointSetCurrent;
