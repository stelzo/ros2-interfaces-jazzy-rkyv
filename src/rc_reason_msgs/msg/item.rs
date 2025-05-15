use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Item {
    pub uuid: ::std::string::String,
    pub grasp_uuids: Vec<::std::string::String>,
    pub r#type: ::std::string::String,
    pub rectangle: crate::rc_reason_msgs::msg::Rectangle,
    pub pose: crate::geometry_msgs::msg::PoseStamped,
}

impl Item {
    pub const RECTANGLE: &'static str = "RECTANGLE";
}

impl Default for Item {
    fn default() -> Self {
        Item {
            uuid: ::std::string::String::new(),
            grasp_uuids: Vec::new(),
            r#type: ::std::string::String::new(),
            rectangle: crate::rc_reason_msgs::msg::Rectangle::default(),
            pose: crate::geometry_msgs::msg::PoseStamped::default(),
        }
    }
}
