use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPlanRequest {
    pub domain: ::std::string::String,
    pub problem: ::std::string::String,
}

impl Default for GetPlanRequest {
    fn default() -> Self {
        GetPlanRequest {
            domain: ::std::string::String::new(),
            problem: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPlanResponse {
    pub success: bool,
    pub plan: crate::plansys2_msgs::msg::Plan,
    pub error_info: ::std::string::String,
}

impl Default for GetPlanResponse {
    fn default() -> Self {
        GetPlanResponse {
            success: false,
            plan: crate::plansys2_msgs::msg::Plan::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct GetPlan;
