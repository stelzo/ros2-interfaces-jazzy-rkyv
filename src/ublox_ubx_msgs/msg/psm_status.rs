use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PSMStatus {
    pub state: u8,
}

impl PSMStatus {
    pub const PSM_STATE_ACQUISITION: u8 = 0;
    pub const PSM_STATE_TRACKING: u8 = 1;
    pub const PSM_STATE_POWER_OPTIMIZED_TRACKING: u8 = 2;
    pub const PSM_STATE_INACTIVE: u8 = 3;
}

impl Default for PSMStatus {
    fn default() -> Self {
        PSMStatus { state: 0 }
    }
}
