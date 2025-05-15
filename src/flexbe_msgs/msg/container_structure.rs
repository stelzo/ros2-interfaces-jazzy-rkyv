use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ContainerStructure {
    pub behavior_id: i32,
    pub containers: Vec<crate::flexbe_msgs::msg::Container>,
}

impl Default for ContainerStructure {
    fn default() -> Self {
        ContainerStructure {
            behavior_id: 0,
            containers: Vec::new(),
        }
    }
}
