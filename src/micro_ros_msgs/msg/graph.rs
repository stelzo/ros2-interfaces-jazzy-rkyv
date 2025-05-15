use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Graph {
    pub nodes: Vec<crate::micro_ros_msgs::msg::Node>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph { nodes: Vec::new() }
    }
}
