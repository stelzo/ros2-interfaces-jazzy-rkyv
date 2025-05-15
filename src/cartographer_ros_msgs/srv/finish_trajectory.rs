use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FinishTrajectoryRequest {
    pub trajectory_id: i32,
}

impl Default for FinishTrajectoryRequest {
    fn default() -> Self {
        FinishTrajectoryRequest { trajectory_id: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FinishTrajectoryResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
}

impl Default for FinishTrajectoryResponse {
    fn default() -> Self {
        FinishTrajectoryResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
        }
    }
}

pub struct FinishTrajectory;
