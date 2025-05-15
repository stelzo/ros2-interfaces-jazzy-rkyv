use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct UInt16Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: u16,
}

impl Default for UInt16Stamped {
    fn default() -> Self {
        UInt16Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
