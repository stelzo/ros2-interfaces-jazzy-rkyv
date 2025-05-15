use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MakePlanFromToRequest {
    pub start: crate::geometry_msgs::msg::Pose,
    pub target: crate::geometry_msgs::msg::Pose,
}

impl Default for MakePlanFromToRequest {
    fn default() -> Self {
        MakePlanFromToRequest {
            start: crate::geometry_msgs::msg::Pose::default(),
            target: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MakePlanFromToResponse {
    pub valid_path_found: bool,
    pub waypoints: crate::mrpt_msgs::msg::WaypointSequence,
}

impl Default for MakePlanFromToResponse {
    fn default() -> Self {
        MakePlanFromToResponse {
            valid_path_found: false,
            waypoints: crate::mrpt_msgs::msg::WaypointSequence::default(),
        }
    }
}

pub struct MakePlanFromTo;
