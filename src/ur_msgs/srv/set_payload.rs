use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPayloadRequest {
    pub mass: f32,
    pub center_of_gravity: crate::geometry_msgs::msg::Vector3,
}

impl Default for SetPayloadRequest {
    fn default() -> Self {
        SetPayloadRequest {
            mass: 0.0,
            center_of_gravity: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetPayloadResponse {
    pub success: bool,
}

impl Default for SetPayloadResponse {
    fn default() -> Self {
        SetPayloadResponse { success: false }
    }
}

pub struct SetPayload;
