use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MutexGroupRequest {
    pub group: ::std::string::String,
    pub claimant: u64,
    pub claim_time: crate::builtin_interfaces::msg::Time,
    pub mode: u32,
}

impl MutexGroupRequest {
    pub const MODE_RELEASE: u32 = 0;
    pub const MODE_LOCK: u32 = 1;
}

impl Default for MutexGroupRequest {
    fn default() -> Self {
        MutexGroupRequest {
            group: ::std::string::String::new(),
            claimant: 0,
            claim_time: crate::builtin_interfaces::msg::Time::default(),
            mode: 0,
        }
    }
}
