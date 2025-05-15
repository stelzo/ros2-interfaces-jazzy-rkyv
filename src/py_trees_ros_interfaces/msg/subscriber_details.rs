use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SubscriberDetails {
    pub topic_name: ::std::string::String,
    pub message_type: ::std::string::String,
    pub latched: bool,
}

impl Default for SubscriberDetails {
    fn default() -> Self {
        SubscriberDetails {
            topic_name: ::std::string::String::new(),
            message_type: ::std::string::String::new(),
            latched: false,
        }
    }
}
