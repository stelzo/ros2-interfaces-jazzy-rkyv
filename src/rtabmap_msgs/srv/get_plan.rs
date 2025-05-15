use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPlanRequest {
    pub goal_node: i32,
    pub goal: crate::geometry_msgs::msg::PoseStamped,
    pub tolerance: f32,
}

impl Default for GetPlanRequest {
    fn default() -> Self {
        GetPlanRequest {
            goal_node: 0,
            goal: crate::geometry_msgs::msg::PoseStamped::default(),
            tolerance: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetPlanResponse {
    pub plan: crate::rtabmap_msgs::msg::Path,
}

impl Default for GetPlanResponse {
    fn default() -> Self {
        GetPlanResponse {
            plan: crate::rtabmap_msgs::msg::Path::default(),
        }
    }
}

pub struct GetPlan;
