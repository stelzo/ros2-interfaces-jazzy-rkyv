use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PipelineState {
    pub request: crate::moveit_msgs::msg::MotionPlanRequest,
    pub response: crate::moveit_msgs::msg::MotionPlanResponse,
    pub pipeline_stage: ::std::string::String,
}

impl Default for PipelineState {
    fn default() -> Self {
        PipelineState {
            request: crate::moveit_msgs::msg::MotionPlanRequest::default(),
            response: crate::moveit_msgs::msg::MotionPlanResponse::default(),
            pipeline_stage: ::std::string::String::new(),
        }
    }
}
