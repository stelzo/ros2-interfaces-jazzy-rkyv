use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MissionRemainingDistanceTime {
    pub remaining_distance: f64,
    pub remaining_time: f64,
}

impl Default for MissionRemainingDistanceTime {
    fn default() -> Self {
        MissionRemainingDistanceTime {
            remaining_distance: 0.0,
            remaining_time: 0.0,
        }
    }
}
