use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Trajectory {
    pub waypoints: Vec<crate::rmf_traffic_msgs::msg::TrajectoryWaypoint>,
}

impl Default for Trajectory {
    fn default() -> Self {
        Trajectory {
            waypoints: Vec::new(),
        }
    }
}
