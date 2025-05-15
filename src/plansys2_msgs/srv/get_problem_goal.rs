use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProblemGoalRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemGoalRequest {
    fn default() -> Self {
        GetProblemGoalRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProblemGoalResponse {
    pub success: bool,
    pub tree: crate::plansys2_msgs::msg::Tree,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemGoalResponse {
    fn default() -> Self {
        GetProblemGoalResponse {
            success: false,
            tree: crate::plansys2_msgs::msg::Tree::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetProblemGoal;
