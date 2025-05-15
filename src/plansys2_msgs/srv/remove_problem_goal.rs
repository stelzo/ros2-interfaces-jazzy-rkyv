use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RemoveProblemGoalRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for RemoveProblemGoalRequest {
    fn default() -> Self {
        RemoveProblemGoalRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RemoveProblemGoalResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for RemoveProblemGoalResponse {
    fn default() -> Self {
        RemoveProblemGoalResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct RemoveProblemGoal;
