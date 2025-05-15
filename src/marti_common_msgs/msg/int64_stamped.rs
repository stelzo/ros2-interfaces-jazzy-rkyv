use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Int64Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: i64,
}

impl Default for Int64Stamped {
    fn default() -> Self {
        Int64Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
