use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddProblemGoalRequest {
    pub tree: crate::plansys2_msgs::msg::Tree,
}

impl Default for AddProblemGoalRequest {
    fn default() -> Self {
        AddProblemGoalRequest {
            tree: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddProblemGoalResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AddProblemGoalResponse {
    fn default() -> Self {
        AddProblemGoalResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct AddProblemGoal;
