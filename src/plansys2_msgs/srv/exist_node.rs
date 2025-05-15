use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ExistNodeRequest {
    pub node: crate::plansys2_msgs::msg::Node,
}

impl Default for ExistNodeRequest {
    fn default() -> Self {
        ExistNodeRequest {
            node: crate::plansys2_msgs::msg::Node::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ExistNodeResponse {
    pub exist: bool,
}

impl Default for ExistNodeResponse {
    fn default() -> Self {
        ExistNodeResponse { exist: false }
    }
}

pub struct ExistNode;
