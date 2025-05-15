use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Tree {
    pub nodes: Vec<crate::plansys2_msgs::msg::Node>,
}

impl Default for Tree {
    fn default() -> Self {
        Tree { nodes: Vec::new() }
    }
}
