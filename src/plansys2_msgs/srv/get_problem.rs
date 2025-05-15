use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProblemRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemRequest {
    fn default() -> Self {
        GetProblemRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProblemResponse {
    pub success: bool,
    pub problem: ::std::string::String,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemResponse {
    fn default() -> Self {
        GetProblemResponse {
            success: false,
            problem: ::std::string::String::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetProblem;
