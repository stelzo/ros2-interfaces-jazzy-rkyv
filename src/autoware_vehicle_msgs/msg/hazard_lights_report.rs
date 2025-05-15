use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HazardLightsReport {
    pub stamp: crate::builtin_interfaces::msg::Time,
    pub report: u8,
}

impl HazardLightsReport {
    pub const DISABLE: u8 = 1;
    pub const ENABLE: u8 = 2;
}

impl Default for HazardLightsReport {
    fn default() -> Self {
        HazardLightsReport {
            stamp: crate::builtin_interfaces::msg::Time::default(),
            report: 0,
        }
    }
}
