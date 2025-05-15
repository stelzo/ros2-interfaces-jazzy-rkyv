use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TrajectoryConstraints {
    pub constraints: Vec<crate::moveit_msgs::msg::Constraints>,
}

impl Default for TrajectoryConstraints {
    fn default() -> Self {
        TrajectoryConstraints {
            constraints: Vec::new(),
        }
    }
}
