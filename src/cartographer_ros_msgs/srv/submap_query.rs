use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SubmapQueryRequest {
    pub trajectory_id: i32,
    pub submap_index: i32,
}

impl Default for SubmapQueryRequest {
    fn default() -> Self {
        SubmapQueryRequest {
            trajectory_id: 0,
            submap_index: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SubmapQueryResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub submap_version: i32,
    pub textures: Vec<crate::cartographer_ros_msgs::msg::SubmapTexture>,
}

impl Default for SubmapQueryResponse {
    fn default() -> Self {
        SubmapQueryResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            submap_version: 0,
            textures: Vec::new(),
        }
    }
}

pub struct SubmapQuery;
