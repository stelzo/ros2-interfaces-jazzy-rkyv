use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PublisherDetails {
    pub topic_name: ::std::string::String,
    pub message_type: ::std::string::String,
    pub latched: bool,
}

impl Default for PublisherDetails {
    fn default() -> Self {
        PublisherDetails {
            topic_name: ::std::string::String::new(),
            message_type: ::std::string::String::new(),
            latched: false,
        }
    }
}
