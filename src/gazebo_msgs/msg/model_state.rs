use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ModelState {
    pub model_name: ::std::string::String,
    pub pose: crate::geometry_msgs::msg::Pose,
    pub twist: crate::geometry_msgs::msg::Twist,
    pub reference_frame: ::std::string::String,
}

impl Default for ModelState {
    fn default() -> Self {
        ModelState {
            model_name: ::std::string::String::new(),
            pose: crate::geometry_msgs::msg::Pose::default(),
            twist: crate::geometry_msgs::msg::Twist::default(),
            reference_frame: ::std::string::String::new(),
        }
    }
}
