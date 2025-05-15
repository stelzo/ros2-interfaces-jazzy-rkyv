use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ItemModel {
    pub r#type: ::std::string::String,
    pub unknown: crate::rc_reason_msgs::msg::RangeBox,
    pub rectangle: crate::rc_reason_msgs::msg::RangeRectangle,
}

impl ItemModel {
    pub const UNKNOWN: &'static str = "UNKNOWN";
    pub const RECTANGLE: &'static str = "RECTANGLE";
}

impl Default for ItemModel {
    fn default() -> Self {
        ItemModel {
            r#type: ::std::string::String::new(),
            unknown: crate::rc_reason_msgs::msg::RangeBox::default(),
            rectangle: crate::rc_reason_msgs::msg::RangeRectangle::default(),
        }
    }
}
