use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SwitchControllerRequest {
    pub activate_controllers: Vec<::std::string::String>,
    pub deactivate_controllers: Vec<::std::string::String>,
    pub strictness: i32,
    pub activate_asap: bool,
    pub timeout: crate::builtin_interfaces::msg::Duration,
}

impl SwitchControllerRequest {
    pub const BEST_EFFORT: i32 = 1;
    pub const STRICT: i32 = 2;
}

impl Default for SwitchControllerRequest {
    fn default() -> Self {
        SwitchControllerRequest {
            activate_controllers: Vec::new(),
            deactivate_controllers: Vec::new(),
            strictness: 0,
            activate_asap: false,
            timeout: crate::builtin_interfaces::msg::Duration::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SwitchControllerResponse {
    pub ok: bool,
}

impl Default for SwitchControllerResponse {
    fn default() -> Self {
        SwitchControllerResponse { ok: false }
    }
}

pub struct SwitchController;
