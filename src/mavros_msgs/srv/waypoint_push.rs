use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointPushRequest {
    pub start_index: u16,
    pub waypoints: Vec<crate::mavros_msgs::msg::Waypoint>,
}

impl Default for WaypointPushRequest {
    fn default() -> Self {
        WaypointPushRequest {
            start_index: 0,
            waypoints: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointPushResponse {
    pub success: bool,
    pub wp_transfered: u32,
}

impl Default for WaypointPushResponse {
    fn default() -> Self {
        WaypointPushResponse {
            success: false,
            wp_transfered: 0,
        }
    }
}

pub struct WaypointPush;
