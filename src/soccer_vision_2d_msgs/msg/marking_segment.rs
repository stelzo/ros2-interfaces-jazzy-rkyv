use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MarkingSegment {
    pub start: crate::vision_msgs::msg::Point2D,
    pub end: crate::vision_msgs::msg::Point2D,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for MarkingSegment {
    fn default() -> Self {
        MarkingSegment {
            start: crate::vision_msgs::msg::Point2D::default(),
            end: crate::vision_msgs::msg::Point2D::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}
