use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TimeStamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: crate::builtin_interfaces::msg::Time,
}

impl Default for TimeStamped {
    fn default() -> Self {
        TimeStamped {
            header: crate::std_msgs::msg::Header::default(),
            value: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}
