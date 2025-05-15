use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GoalID {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub id: ::std::string::String,
}

impl Default for GoalID {
    fn default() -> Self {
        GoalID {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            id: ::std::string::String::new(),
        }
    }
}
