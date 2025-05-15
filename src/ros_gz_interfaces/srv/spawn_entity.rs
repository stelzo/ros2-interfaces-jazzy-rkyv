use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnEntityRequest {
    pub entity_factory: crate::ros_gz_interfaces::msg::EntityFactory,
}

impl Default for SpawnEntityRequest {
    fn default() -> Self {
        SpawnEntityRequest {
            entity_factory: crate::ros_gz_interfaces::msg::EntityFactory::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SpawnEntityResponse {
    pub success: bool,
}

impl Default for SpawnEntityResponse {
    fn default() -> Self {
        SpawnEntityResponse { success: false }
    }
}

pub struct SpawnEntity;
