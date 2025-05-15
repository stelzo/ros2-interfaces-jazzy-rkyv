use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Int8Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: i8,
}

impl Default for Int8Stamped {
    fn default() -> Self {
        Int8Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
