use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetCMGraphRequest {
    pub node_ids: Vec<u64>,
}

impl Default for GetCMGraphRequest {
    fn default() -> Self {
        GetCMGraphRequest {
            node_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetCMGraphResponse {
    pub cm_graph: crate::mrpt_msgs::msg::NetworkOfPoses,
}

impl Default for GetCMGraphResponse {
    fn default() -> Self {
        GetCMGraphResponse {
            cm_graph: crate::mrpt_msgs::msg::NetworkOfPoses::default(),
        }
    }
}

pub struct GetCMGraph;
