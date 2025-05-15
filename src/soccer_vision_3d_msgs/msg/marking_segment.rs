use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MarkingSegment {
    pub start: crate::geometry_msgs::msg::Point,
    pub end: crate::geometry_msgs::msg::Point,
    pub confidence: crate::soccer_vision_attribute_msgs::msg::Confidence,
}

impl Default for MarkingSegment {
    fn default() -> Self {
        MarkingSegment {
            start: crate::geometry_msgs::msg::Point::default(),
            end: crate::geometry_msgs::msg::Point::default(),
            confidence: crate::soccer_vision_attribute_msgs::msg::Confidence::default(),
        }
    }
}
