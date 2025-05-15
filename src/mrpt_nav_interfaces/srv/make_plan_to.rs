use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MakePlanToRequest {
    pub target: crate::geometry_msgs::msg::PoseStamped,
}

impl Default for MakePlanToRequest {
    fn default() -> Self {
        MakePlanToRequest {
            target: crate::geometry_msgs::msg::PoseStamped::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MakePlanToResponse {
    pub valid_path_found: bool,
    pub waypoints: crate::mrpt_msgs::msg::WaypointSequence,
}

impl Default for MakePlanToResponse {
    fn default() -> Self {
        MakePlanToResponse {
            valid_path_found: false,
            waypoints: crate::mrpt_msgs::msg::WaypointSequence::default(),
        }
    }
}

pub struct MakePlanTo;
