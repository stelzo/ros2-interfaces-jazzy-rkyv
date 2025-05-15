use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TrajectoryWaypoint {
    pub time: i64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
}

impl Default for TrajectoryWaypoint {
    fn default() -> Self {
        TrajectoryWaypoint {
            time: 0,
            position: [0.0; 3],
            velocity: [0.0; 3],
        }
    }
}
