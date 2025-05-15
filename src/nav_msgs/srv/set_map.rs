use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetMapRequest {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetMapRequest {
    fn default() -> Self {
        SetMapRequest {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
            initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetMapResponse {
    pub success: bool,
}

impl Default for SetMapResponse {
    fn default() -> Self {
        SetMapResponse { success: false }
    }
}

pub struct SetMap;
