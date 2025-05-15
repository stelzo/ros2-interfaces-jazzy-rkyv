use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetNodeDetailsRequest {
    pub expression: ::std::string::String,
}

impl Default for GetNodeDetailsRequest {
    fn default() -> Self {
        GetNodeDetailsRequest {
            expression: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetNodeDetailsResponse {
    pub success: bool,
    pub node: crate::plansys2_msgs::msg::Node,
    pub error_info: ::std::string::String,
}

impl Default for GetNodeDetailsResponse {
    fn default() -> Self {
        GetNodeDetailsResponse {
            success: false,
            node: crate::plansys2_msgs::msg::Node::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetNodeDetails;
