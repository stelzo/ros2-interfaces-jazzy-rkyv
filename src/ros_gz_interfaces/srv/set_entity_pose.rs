use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetEntityPoseRequest {
    pub entity: crate::ros_gz_interfaces::msg::Entity,
    pub pose: crate::geometry_msgs::msg::Pose,
}

impl Default for SetEntityPoseRequest {
    fn default() -> Self {
        SetEntityPoseRequest {
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
            pose: crate::geometry_msgs::msg::Pose::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetEntityPoseResponse {
    pub success: bool,
}

impl Default for SetEntityPoseResponse {
    fn default() -> Self {
        SetEntityPoseResponse { success: false }
    }
}

pub struct SetEntityPose;
