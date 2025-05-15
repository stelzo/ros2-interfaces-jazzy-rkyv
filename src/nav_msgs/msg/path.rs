use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Path {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::geometry_msgs::msg::PoseStamped>,
}

impl Default for Path {
    fn default() -> Self {
        Path {
            header: crate::std_msgs::msg::Header::default(),
            poses: Vec::new(),
        }
    }
}
