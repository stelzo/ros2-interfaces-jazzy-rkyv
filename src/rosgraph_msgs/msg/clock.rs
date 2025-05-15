use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Clock {
    pub clock: crate::builtin_interfaces::msg::Time,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            clock: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}
