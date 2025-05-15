use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ObjectHypothesisWithPose {
    pub hypothesis: crate::vision_msgs::msg::ObjectHypothesis,
    pub pose: crate::geometry_msgs::msg::PoseWithCovariance,
}

impl Default for ObjectHypothesisWithPose {
    fn default() -> Self {
        ObjectHypothesisWithPose {
            hypothesis: crate::vision_msgs::msg::ObjectHypothesis::default(),
            pose: crate::geometry_msgs::msg::PoseWithCovariance::default(),
        }
    }
}
