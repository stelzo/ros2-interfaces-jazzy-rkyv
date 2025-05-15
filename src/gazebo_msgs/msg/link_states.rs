use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LinkStates {
    pub name: Vec<::std::string::String>,
    pub pose: Vec<crate::geometry_msgs::msg::Pose>,
    pub twist: Vec<crate::geometry_msgs::msg::Twist>,
}

impl Default for LinkStates {
    fn default() -> Self {
        LinkStates {
            name: Vec::new(),
            pose: Vec::new(),
            twist: Vec::new(),
        }
    }
}
