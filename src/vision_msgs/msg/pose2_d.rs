use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Pose2D {
    pub position: crate::vision_msgs::msg::Point2D,
    pub theta: f64,
}

impl Default for Pose2D {
    fn default() -> Self {
        Pose2D {
            position: crate::vision_msgs::msg::Point2D::default(),
            theta: 0.0,
        }
    }
}
