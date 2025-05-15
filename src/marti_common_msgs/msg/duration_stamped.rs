use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DurationStamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: crate::builtin_interfaces::msg::Duration,
}

impl Default for DurationStamped {
    fn default() -> Self {
        DurationStamped {
            header: crate::std_msgs::msg::Header::default(),
            value: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}
