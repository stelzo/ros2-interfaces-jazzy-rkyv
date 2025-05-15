use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StringVec {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<::std::string::String>,
}

impl Default for StringVec {
    fn default() -> Self {
        StringVec {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}
