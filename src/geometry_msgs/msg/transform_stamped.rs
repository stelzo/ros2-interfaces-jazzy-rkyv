use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TransformStamped {
    pub header: crate::std_msgs::msg::Header,
    pub child_frame_id: ::std::string::String,
    pub transform: crate::geometry_msgs::msg::Transform,
}

impl Default for TransformStamped {
    fn default() -> Self {
        TransformStamped {
            header: crate::std_msgs::msg::Header::default(),
            child_frame_id: ::std::string::String::new(),
            transform: crate::geometry_msgs::msg::Transform::default(),
        }
    }
}
