use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DispatchCommand {
    pub fleet_name: ::std::string::String,
    pub task_id: ::std::string::String,
    pub dispatch_id: u64,
    pub timestamp: crate::builtin_interfaces::msg::Time,
    pub r#type: u8,
}

impl DispatchCommand {
    pub const TYPE_AWARD: u8 = 1;
    pub const TYPE_REMOVE: u8 = 2;
}

impl Default for DispatchCommand {
    fn default() -> Self {
        DispatchCommand {
            fleet_name: ::std::string::String::new(),
            task_id: ::std::string::String::new(),
            dispatch_id: 0,
            timestamp: crate::builtin_interfaces::msg::Time::default(),
            r#type: 0,
        }
    }
}
