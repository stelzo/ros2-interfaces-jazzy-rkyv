use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MotionSequenceResponse {
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
    pub sequence_start: crate::moveit_msgs::msg::RobotState,
    pub planned_trajectories: Vec<crate::moveit_msgs::msg::RobotTrajectory>,
    pub planning_time: f64,
}

impl Default for MotionSequenceResponse {
    fn default() -> Self {
        MotionSequenceResponse {
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
            sequence_start: crate::moveit_msgs::msg::RobotState::default(),
            planned_trajectories: Vec::new(),
            planning_time: 0.0,
        }
    }
}
