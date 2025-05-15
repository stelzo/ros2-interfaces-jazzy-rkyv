use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PoseArray {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::geometry_msgs::msg::Pose>,
}

impl Default for PoseArray {
    fn default() -> Self {
        PoseArray {
            header: crate::std_msgs::msg::Header::default(),
            poses: Vec::new(),
        }
    }
}
