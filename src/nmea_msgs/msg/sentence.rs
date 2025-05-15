use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Sentence {
    pub header: crate::std_msgs::msg::Header,
    pub sentence: ::std::string::String,
}

impl Default for Sentence {
    fn default() -> Self {
        Sentence {
            header: crate::std_msgs::msg::Header::default(),
            sentence: ::std::string::String::new(),
        }
    }
}
