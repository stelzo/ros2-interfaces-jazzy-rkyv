use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScenarioStatus {
    pub system_time: crate::builtin_interfaces::msg::Time,
    pub ros_time: crate::builtin_interfaces::msg::Time,
    pub data: ::std::string::String,
}

impl Default for ScenarioStatus {
    fn default() -> Self {
        ScenarioStatus {
            system_time: crate::builtin_interfaces::msg::Time::default(),
            ros_time: crate::builtin_interfaces::msg::Time::default(),
            data: ::std::string::String::new(),
        }
    }
}
