use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EntityWrench {
    pub header: crate::std_msgs::msg::Header,
    pub entity: crate::ros_gz_interfaces::msg::Entity,
    pub wrench: crate::geometry_msgs::msg::Wrench,
}

impl Default for EntityWrench {
    fn default() -> Self {
        EntityWrench {
            header: crate::std_msgs::msg::Header::default(),
            entity: crate::ros_gz_interfaces::msg::Entity::default(),
            wrench: crate::geometry_msgs::msg::Wrench::default(),
        }
    }
}
