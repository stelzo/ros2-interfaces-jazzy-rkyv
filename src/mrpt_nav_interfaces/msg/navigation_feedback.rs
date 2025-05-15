use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NavigationFeedback {
    pub total_waypoints: i16,
    pub reached_waypoints: i16,
}

impl Default for NavigationFeedback {
    fn default() -> Self {
        NavigationFeedback {
            total_waypoints: 0,
            reached_waypoints: 0,
        }
    }
}
