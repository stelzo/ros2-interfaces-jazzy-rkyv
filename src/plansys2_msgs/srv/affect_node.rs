use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AffectNodeRequest {
    pub node: crate::plansys2_msgs::msg::Node,
}

impl Default for AffectNodeRequest {
    fn default() -> Self {
        AffectNodeRequest {
            node: crate::plansys2_msgs::msg::Node::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AffectNodeResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AffectNodeResponse {
    fn default() -> Self {
        AffectNodeResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct AffectNode;
