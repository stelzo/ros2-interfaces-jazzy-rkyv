use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Path {
    pub header: crate::std_msgs::msg::Header,
    pub node_ids: Vec<i32>,
    pub poses: Vec<crate::geometry_msgs::msg::Pose>,
}

impl Default for Path {
    fn default() -> Self {
        Path {
            header: crate::std_msgs::msg::Header::default(),
            node_ids: Vec::new(),
            poses: Vec::new(),
        }
    }
}
