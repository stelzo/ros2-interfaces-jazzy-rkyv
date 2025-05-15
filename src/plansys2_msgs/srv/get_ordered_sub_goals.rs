use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetOrderedSubGoalsRequest {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetOrderedSubGoalsRequest {
    fn default() -> Self {
        GetOrderedSubGoalsRequest {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetOrderedSubGoalsResponse {
    pub success: bool,
    pub sub_goals: Vec<crate::plansys2_msgs::msg::Tree>,
    pub error_info: ::std::string::String,
}

impl Default for GetOrderedSubGoalsResponse {
    fn default() -> Self {
        GetOrderedSubGoalsResponse {
            success: false,
            sub_goals: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetOrderedSubGoals;
