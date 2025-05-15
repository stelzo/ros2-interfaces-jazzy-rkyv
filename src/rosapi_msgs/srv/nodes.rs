use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NodesRequest {}

impl Default for NodesRequest {
    fn default() -> Self {
        NodesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NodesResponse {
    pub nodes: Vec<::std::string::String>,
}

impl Default for NodesResponse {
    fn default() -> Self {
        NodesResponse { nodes: Vec::new() }
    }
}

pub struct Nodes;
