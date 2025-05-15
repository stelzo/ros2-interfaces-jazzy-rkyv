use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MotionSequenceRequest {
    pub items: Vec<crate::moveit_msgs::msg::MotionSequenceItem>,
}

impl Default for MotionSequenceRequest {
    fn default() -> Self {
        MotionSequenceRequest { items: Vec::new() }
    }
}
