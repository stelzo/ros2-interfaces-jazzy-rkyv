use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UInt8Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: u8,
}

impl Default for UInt8Stamped {
    fn default() -> Self {
        UInt8Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
