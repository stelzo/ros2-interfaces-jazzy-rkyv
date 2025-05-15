use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ControlModeReport {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub mode: u8,
}

impl ControlModeReport {
    pub const NO_COMMAND: u8 = 0;
    pub const AUTONOMOUS: u8 = 1;
    pub const AUTONOMOUS_STEER_ONLY: u8 = 2;
    pub const AUTONOMOUS_VELOCITY_ONLY: u8 = 3;
    pub const MANUAL: u8 = 4;
    pub const DISENGAGED: u8 = 5;
    pub const NOT_READY: u8 = 6;
}

impl Default for ControlModeReport {
    fn default() -> Self {
        ControlModeReport {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            mode: 0,
        }
    }
}
