use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Ball {
    pub bb: crate::vision_msgs::msg::BoundingBox2D,
    pub center: crate::vision_msgs::msg::Point2D,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            bb: crate::vision_msgs::msg::BoundingBox2D::default(),
            center: crate::vision_msgs::msg::Point2D::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}
