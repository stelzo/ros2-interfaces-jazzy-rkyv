use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Engage {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub engage: bool,
}

impl Default for Engage {
    fn default() -> Self {
        Engage {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            engage: false,
        }
    }
}
