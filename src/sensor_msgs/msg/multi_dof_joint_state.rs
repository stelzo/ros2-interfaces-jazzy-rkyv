use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MultiDOFJointState {
    pub header: crate::std_msgs::msg::Header,
    pub joint_names: Vec<::std::string::String>,
    pub transforms: Vec<crate::geometry_msgs::msg::Transform>,
    pub twist: Vec<crate::geometry_msgs::msg::Twist>,
    pub wrench: Vec<crate::geometry_msgs::msg::Wrench>,
}

impl Default for MultiDOFJointState {
    fn default() -> Self {
        MultiDOFJointState {
            header: crate::std_msgs::msg::Header::default(),
            joint_names: Vec::new(),
            transforms: Vec::new(),
            twist: Vec::new(),
            wrench: Vec::new(),
        }
    }
}
