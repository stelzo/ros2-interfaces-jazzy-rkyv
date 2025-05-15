use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetJointTrajectoryRequest {
    pub model_name: ::std::string::String,
    pub joint_trajectory: crate::trajectory_msgs::msg::JointTrajectory,
    pub model_pose: crate::geometry_msgs::msg::Pose,
    pub set_model_pose: bool,
    pub disable_physics_updates: bool,
}

impl Default for SetJointTrajectoryRequest {
    fn default() -> Self {
        SetJointTrajectoryRequest {
            model_name: ::std::string::String::new(),
            joint_trajectory: crate::trajectory_msgs::msg::JointTrajectory::default(),
            model_pose: crate::geometry_msgs::msg::Pose::default(),
            set_model_pose: false,
            disable_physics_updates: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetJointTrajectoryResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for SetJointTrajectoryResponse {
    fn default() -> Self {
        SetJointTrajectoryResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct SetJointTrajectory;
