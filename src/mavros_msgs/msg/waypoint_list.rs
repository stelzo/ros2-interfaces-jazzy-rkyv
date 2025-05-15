use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WaypointList {
    pub current_seq: u16,
    pub waypoints: Vec<crate::mavros_msgs::msg::Waypoint>,
}

impl Default for WaypointList {
    fn default() -> Self {
        WaypointList {
            current_seq: 0,
            waypoints: Vec::new(),
        }
    }
}
