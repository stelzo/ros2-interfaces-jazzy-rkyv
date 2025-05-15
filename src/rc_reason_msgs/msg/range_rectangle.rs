use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RangeRectangle {
    pub min_dimensions: crate::rc_reason_msgs::msg::Rectangle,
    pub max_dimensions: crate::rc_reason_msgs::msg::Rectangle,
}

impl Default for RangeRectangle {
    fn default() -> Self {
        RangeRectangle {
            min_dimensions: crate::rc_reason_msgs::msg::Rectangle::default(),
            max_dimensions: crate::rc_reason_msgs::msg::Rectangle::default(),
        }
    }
}
