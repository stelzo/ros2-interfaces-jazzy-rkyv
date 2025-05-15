use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TrajectoryQueryRequest {
    pub trajectory_id: i32,
}

impl Default for TrajectoryQueryRequest {
    fn default() -> Self {
        TrajectoryQueryRequest { trajectory_id: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TrajectoryQueryResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub trajectory: Vec<crate::geometry_msgs::msg::PoseStamped>,
}

impl Default for TrajectoryQueryResponse {
    fn default() -> Self {
        TrajectoryQueryResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            trajectory: Vec::new(),
        }
    }
}

pub struct TrajectoryQuery;
