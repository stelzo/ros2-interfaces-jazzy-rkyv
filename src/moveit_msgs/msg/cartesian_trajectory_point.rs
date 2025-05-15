use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CartesianTrajectoryPoint {
    pub point: crate::moveit_msgs::msg::CartesianPoint,
    pub time_from_start: crate::builtin_interfaces::msg::Duration,
}

impl Default for CartesianTrajectoryPoint {
    fn default() -> Self {
        CartesianTrajectoryPoint {
            point: crate::moveit_msgs::msg::CartesianPoint::default(),
            time_from_start: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}
