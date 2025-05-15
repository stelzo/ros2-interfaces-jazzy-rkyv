use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetGraphRequest {
    pub graph: crate::fuse_msgs::msg::SerializedGraph,
}

impl Default for SetGraphRequest {
    fn default() -> Self {
        SetGraphRequest {
            graph: crate::fuse_msgs::msg::SerializedGraph::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetGraphResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetGraphResponse {
    fn default() -> Self {
        SetGraphResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct SetGraph;
