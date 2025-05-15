use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RobotTrajectory {
    pub joint_trajectory: crate::trajectory_msgs::msg::JointTrajectory,
    pub multi_dof_joint_trajectory: crate::trajectory_msgs::msg::MultiDOFJointTrajectory,
}

impl Default for RobotTrajectory {
    fn default() -> Self {
        RobotTrajectory {
            joint_trajectory: crate::trajectory_msgs::msg::JointTrajectory::default(),
            multi_dof_joint_trajectory:
                crate::trajectory_msgs::msg::MultiDOFJointTrajectory::default(),
        }
    }
}
