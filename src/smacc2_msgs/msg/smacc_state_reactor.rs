use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SmaccStateReactor {
    pub index: i32,
    pub type_name: ::std::string::String,
    pub object_tag: ::std::string::String,
    pub event_sources: Vec<crate::smacc2_msgs::msg::SmaccEvent>,
}

impl Default for SmaccStateReactor {
    fn default() -> Self {
        SmaccStateReactor {
            index: 0,
            type_name: ::std::string::String::new(),
            object_tag: ::std::string::String::new(),
            event_sources: Vec::new(),
        }
    }
}
