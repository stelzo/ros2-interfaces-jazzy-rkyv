use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HazardLightsCommand {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub command: u8,
}

impl HazardLightsCommand {
    pub const NO_COMMAND: u8 = 0;
    pub const DISABLE: u8 = 1;
    pub const ENABLE: u8 = 2;
}

impl Default for HazardLightsCommand {
    fn default() -> Self {
        HazardLightsCommand {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            command: 0,
        }
    }
}
