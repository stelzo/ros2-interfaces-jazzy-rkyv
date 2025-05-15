use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMotionSequenceRequest {
    pub request: crate::moveit_msgs::msg::MotionSequenceRequest,
}

impl Default for GetMotionSequenceRequest {
    fn default() -> Self {
        GetMotionSequenceRequest {
            request: crate::moveit_msgs::msg::MotionSequenceRequest::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetMotionSequenceResponse {
    pub response: crate::moveit_msgs::msg::MotionSequenceResponse,
}

impl Default for GetMotionSequenceResponse {
    fn default() -> Self {
        GetMotionSequenceResponse {
            response: crate::moveit_msgs::msg::MotionSequenceResponse::default(),
        }
    }
}

pub struct GetMotionSequence;
