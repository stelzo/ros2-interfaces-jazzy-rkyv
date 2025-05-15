use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RCOut {
    pub header: crate::std_msgs::msg::Header,
    pub channels: Vec<u16>,
}

impl Default for RCOut {
    fn default() -> Self {
        RCOut {
            header: crate::std_msgs::msg::Header::default(),
            channels: Vec::new(),
        }
    }
}
