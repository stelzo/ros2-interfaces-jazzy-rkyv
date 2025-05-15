use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MotionSequenceItem {
    pub req: crate::moveit_msgs::msg::MotionPlanRequest,
    pub blend_radius: f64,
}

impl Default for MotionSequenceItem {
    fn default() -> Self {
        MotionSequenceItem {
            req: crate::moveit_msgs::msg::MotionPlanRequest::default(),
            blend_radius: 0.0,
        }
    }
}
