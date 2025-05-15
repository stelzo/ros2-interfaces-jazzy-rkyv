use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LoadCarrier {
    pub id: ::std::string::String,
    pub r#type: ::std::string::String,
    pub outer_dimensions: crate::rc_reason_msgs::msg::Box,
    pub inner_dimensions: crate::rc_reason_msgs::msg::Box,
    pub rim_thickness: crate::rc_reason_msgs::msg::Rectangle,
    pub rim_step_height: f64,
    pub rim_ledge: crate::rc_reason_msgs::msg::Rectangle,
    pub height_open_side: f64,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
    pub overfilled: bool,
}

impl Default for LoadCarrier {
    fn default() -> Self {
        LoadCarrier {
            id: ::std::string::String::new(),
            r#type: ::std::string::String::new(),
            outer_dimensions: crate::rc_reason_msgs::msg::Box::default(),
            inner_dimensions: crate::rc_reason_msgs::msg::Box::default(),
            rim_thickness: crate::rc_reason_msgs::msg::Rectangle::default(),
            rim_step_height: 0.0,
            rim_ledge: crate::rc_reason_msgs::msg::Rectangle::default(),
            height_open_side: 0.0,
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
            overfilled: false,
        }
    }
}
