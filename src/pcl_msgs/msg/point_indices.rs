use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PointIndices {
    pub header: crate::std_msgs::msg::Header,
    pub indices: Vec<i32>,
}

impl Default for PointIndices {
    fn default() -> Self {
        PointIndices {
            header: crate::std_msgs::msg::Header::default(),
            indices: Vec::new(),
        }
    }
}
