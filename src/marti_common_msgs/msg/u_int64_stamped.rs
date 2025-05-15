use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UInt64Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: u64,
}

impl Default for UInt64Stamped {
    fn default() -> Self {
        UInt64Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
