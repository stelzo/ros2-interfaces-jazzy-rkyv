use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StringArrayStamped {
    pub header: crate::std_msgs::msg::Header,
    pub strings: Vec<::std::string::String>,
}

impl Default for StringArrayStamped {
    fn default() -> Self {
        StringArrayStamped {
            header: crate::std_msgs::msg::Header::default(),
            strings: Vec::new(),
        }
    }
}
