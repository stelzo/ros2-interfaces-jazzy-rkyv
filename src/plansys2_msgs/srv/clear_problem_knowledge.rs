use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ClearProblemKnowledgeRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for ClearProblemKnowledgeRequest {
    fn default() -> Self {
        ClearProblemKnowledgeRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ClearProblemKnowledgeResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for ClearProblemKnowledgeResponse {
    fn default() -> Self {
        ClearProblemKnowledgeResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct ClearProblemKnowledge;
