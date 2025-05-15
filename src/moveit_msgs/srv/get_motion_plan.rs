use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMotionPlanRequest {
    pub motion_plan_request: crate::moveit_msgs::msg::MotionPlanRequest,
}

impl Default for GetMotionPlanRequest {
    fn default() -> Self {
        GetMotionPlanRequest {
            motion_plan_request: crate::moveit_msgs::msg::MotionPlanRequest::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMotionPlanResponse {
    pub motion_plan_response: crate::moveit_msgs::msg::MotionPlanResponse,
}

impl Default for GetMotionPlanResponse {
    fn default() -> Self {
        GetMotionPlanResponse {
            motion_plan_response: crate::moveit_msgs::msg::MotionPlanResponse::default(),
        }
    }
}

pub struct GetMotionPlan;
