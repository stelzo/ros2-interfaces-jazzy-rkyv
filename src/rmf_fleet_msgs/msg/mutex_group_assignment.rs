use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MutexGroupAssignment {
    pub group: ::std::string::String,
    pub claimant: u64,
    pub claim_time: crate::builtin_interfaces::msg::Time,
}

impl Default for MutexGroupAssignment {
    fn default() -> Self {
        MutexGroupAssignment {
            group: ::std::string::String::new(),
            claimant: 0,
            claim_time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}
