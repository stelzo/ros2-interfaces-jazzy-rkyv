use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UInt32Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: u32,
}

impl Default for UInt32Stamped {
    fn default() -> Self {
        UInt32Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
