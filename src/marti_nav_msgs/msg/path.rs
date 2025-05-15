use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Path {
    pub header: crate::std_msgs::msg::Header,
    pub points: Vec<crate::marti_nav_msgs::msg::PathPoint>,
    pub in_reverse: bool,
}

impl Default for Path {
    fn default() -> Self {
        Path {
            header: crate::std_msgs::msg::Header::default(),
            points: Vec::new(),
            in_reverse: false,
        }
    }
}
