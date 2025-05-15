use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListNodesRequest {}

impl Default for ListNodesRequest {
    fn default() -> Self {
        ListNodesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListNodesResponse {
    pub full_node_names: Vec<::std::string::String>,
    pub unique_ids: Vec<u64>,
}

impl Default for ListNodesResponse {
    fn default() -> Self {
        ListNodesResponse {
            full_node_names: Vec::new(),
            unique_ids: Vec::new(),
        }
    }
}

pub struct ListNodes;
