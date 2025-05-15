use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Session {
    pub request_time: crate::builtin_interfaces::msg::Time,
    pub requester_id: ::std::string::String,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            request_time: crate::builtin_interfaces::msg::Time::default(),
            requester_id: ::std::string::String::new(),
        }
    }
}
