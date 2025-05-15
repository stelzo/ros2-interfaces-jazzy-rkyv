use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct QueryPlannerInterfacesRequest {}

impl Default for QueryPlannerInterfacesRequest {
    fn default() -> Self {
        QueryPlannerInterfacesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct QueryPlannerInterfacesResponse {
    pub planner_interfaces: Vec<crate::moveit_msgs::msg::PlannerInterfaceDescription>,
}

impl Default for QueryPlannerInterfacesResponse {
    fn default() -> Self {
        QueryPlannerInterfacesResponse {
            planner_interfaces: Vec::new(),
        }
    }
}

pub struct QueryPlannerInterfaces;
