use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Metadata {
    pub header: crate::std_msgs::msg::Header,
    pub json_data: ::std::string::String,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            header: crate::std_msgs::msg::Header::default(),
            json_data: ::std::string::String::new(),
        }
    }
}
