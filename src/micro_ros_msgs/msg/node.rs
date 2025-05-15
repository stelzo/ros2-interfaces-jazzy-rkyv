use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Node {
    pub node_namespace: ::std::string::String,
    pub node_name: ::std::string::String,
    pub entities: Vec<crate::micro_ros_msgs::msg::Entity>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            node_namespace: ::std::string::String::new(),
            node_name: ::std::string::String::new(),
            entities: Vec::new(),
        }
    }
}
