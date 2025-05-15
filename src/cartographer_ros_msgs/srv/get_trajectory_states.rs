use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetTrajectoryStatesRequest {}

impl Default for GetTrajectoryStatesRequest {
    fn default() -> Self {
        GetTrajectoryStatesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetTrajectoryStatesResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub trajectory_states: crate::cartographer_ros_msgs::msg::TrajectoryStates,
}

impl Default for GetTrajectoryStatesResponse {
    fn default() -> Self {
        GetTrajectoryStatesResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            trajectory_states: crate::cartographer_ros_msgs::msg::TrajectoryStates::default(),
        }
    }
}

pub struct GetTrajectoryStates;
