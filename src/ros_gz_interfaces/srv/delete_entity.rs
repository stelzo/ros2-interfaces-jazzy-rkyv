use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteEntityRequest {
    pub entity: crate::ros_gz_interfaces::msg::Entity,
}

impl Default for DeleteEntityRequest {
    fn default() -> Self {
        DeleteEntityRequest {
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteEntityResponse {
    pub success: bool,
}

impl Default for DeleteEntityResponse {
    fn default() -> Self {
        DeleteEntityResponse { success: false }
    }
}

pub struct DeleteEntity;
