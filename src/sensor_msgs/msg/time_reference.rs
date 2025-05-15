use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TimeReference {
    pub header: crate::std_msgs::msg::Header,
    pub time_ref: crate::builtin_interfaces::msg::Time,
    pub source: ::std::string::String,
}

impl Default for TimeReference {
    fn default() -> Self {
        TimeReference {
            header: crate::std_msgs::msg::Header::default(),
            time_ref: crate::builtin_interfaces::msg::Time::default(),
            source: ::std::string::String::new(),
        }
    }
}
