use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Int32Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: i32,
}

impl Default for Int32Stamped {
    fn default() -> Self {
        Int32Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
