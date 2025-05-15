use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScheduleIdentity {
    pub node_uuid: ::std::string::String,
    pub timestamp: crate::builtin_interfaces::msg::Time,
}

impl Default for ScheduleIdentity {
    fn default() -> Self {
        ScheduleIdentity {
            node_uuid: ::std::string::String::new(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}
