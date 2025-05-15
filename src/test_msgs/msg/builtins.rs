use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Builtins {
    pub duration_value: crate::builtin_interfaces::msg::Duration,
    pub time_value: crate::builtin_interfaces::msg::Time,
}

impl Default for Builtins {
    fn default() -> Self {
        Builtins {
            duration_value: crate::builtin_interfaces::msg::Duration::default(),
            time_value: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}
