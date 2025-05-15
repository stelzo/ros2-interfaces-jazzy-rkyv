use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsProblemGoalSatisfiedRequest {
    pub tree: crate::plansys2_msgs::msg::Tree,
}

impl Default for IsProblemGoalSatisfiedRequest {
    fn default() -> Self {
        IsProblemGoalSatisfiedRequest {
            tree: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsProblemGoalSatisfiedResponse {
    pub success: bool,
    pub satisfied: bool,
    pub error_info: ::std::string::String,
}

impl Default for IsProblemGoalSatisfiedResponse {
    fn default() -> Self {
        IsProblemGoalSatisfiedResponse {
            success: false,
            satisfied: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct IsProblemGoalSatisfied;
