use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Int16Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: i16,
}

impl Default for Int16Stamped {
    fn default() -> Self {
        Int16Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
