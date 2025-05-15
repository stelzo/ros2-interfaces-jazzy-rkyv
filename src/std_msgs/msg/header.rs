use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Header {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub frame_id: ::std::string::String,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            frame_id: ::std::string::String::new(),
        }
    }
}
