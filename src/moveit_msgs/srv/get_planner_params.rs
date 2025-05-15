use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPlannerParamsRequest {
    pub pipeline_id: ::std::string::String,
    pub planner_config: ::std::string::String,
    pub group: ::std::string::String,
}

impl Default for GetPlannerParamsRequest {
    fn default() -> Self {
        GetPlannerParamsRequest {
            pipeline_id: ::std::string::String::new(),
            planner_config: ::std::string::String::new(),
            group: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPlannerParamsResponse {
    pub params: crate::moveit_msgs::msg::PlannerParams,
}

impl Default for GetPlannerParamsResponse {
    fn default() -> Self {
        GetPlannerParamsResponse {
            params: crate::moveit_msgs::msg::PlannerParams::default(),
        }
    }
}

pub struct GetPlannerParams;
