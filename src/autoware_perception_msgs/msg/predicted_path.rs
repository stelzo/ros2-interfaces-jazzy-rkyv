use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PredictedPath {
    pub path: Vec<crate::geometry_msgs::msg::Pose>,
    pub time_step: crate::builtin_interfaces::msg::Duration,
    pub confidence: f32,
}

impl Default for PredictedPath {
    fn default() -> Self {
        PredictedPath {
            path: Vec::new(),
            time_step: crate::builtin_interfaces::msg::Duration::default(),
            confidence: 0.0,
        }
    }
}
