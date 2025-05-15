use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DisplayTrajectory {
    pub model_id: ::std::string::String,
    pub trajectory: Vec<crate::moveit_msgs::msg::RobotTrajectory>,
    pub trajectory_start: crate::moveit_msgs::msg::RobotState,
}

impl Default for DisplayTrajectory {
    fn default() -> Self {
        DisplayTrajectory {
            model_id: ::std::string::String::new(),
            trajectory: Vec::new(),
            trajectory_start: crate::moveit_msgs::msg::RobotState::default(),
        }
    }
}
