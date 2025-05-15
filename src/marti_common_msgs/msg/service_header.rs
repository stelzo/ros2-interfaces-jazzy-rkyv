use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ServiceHeader {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub sequence: u32,
    pub sender: ::std::string::String,
    pub result: bool,
}

impl Default for ServiceHeader {
    fn default() -> Self {
        ServiceHeader {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            sequence: 0,
            sender: ::std::string::String::new(),
            result: false,
        }
    }
}
