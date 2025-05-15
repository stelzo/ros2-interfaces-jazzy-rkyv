use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SubmapList {
    pub header: crate::std_msgs::msg::Header,
    pub submap: Vec<crate::cartographer_ros_msgs::msg::SubmapEntry>,
}

impl Default for SubmapList {
    fn default() -> Self {
        SubmapList {
            header: crate::std_msgs::msg::Header::default(),
            submap: Vec::new(),
        }
    }
}
