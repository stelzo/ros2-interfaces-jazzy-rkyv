use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointPullRequest {}

impl Default for WaypointPullRequest {
    fn default() -> Self {
        WaypointPullRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointPullResponse {
    pub success: bool,
    pub wp_received: u32,
}

impl Default for WaypointPullResponse {
    fn default() -> Self {
        WaypointPullResponse {
            success: false,
            wp_received: 0,
        }
    }
}

pub struct WaypointPull;
