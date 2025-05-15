use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApplyJointEffortRequest {
    pub joint_name: ::std::string::String,
    pub effort: f64,
    pub start_time: crate::builtin_interfaces::msg::Time,
    pub duration: crate::builtin_interfaces::msg::Duration,
}

impl Default for ApplyJointEffortRequest {
    fn default() -> Self {
        ApplyJointEffortRequest {
            joint_name: ::std::string::String::new(),
            effort: 0.0,
            start_time: crate::builtin_interfaces::msg::Time::default(),
            duration: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ApplyJointEffortResponse {
    pub success: bool,
    pub status_message: ::std::string::String,
}

impl Default for ApplyJointEffortResponse {
    fn default() -> Self {
        ApplyJointEffortResponse {
            success: false,
            status_message: ::std::string::String::new(),
        }
    }
}

pub struct ApplyJointEffort;
