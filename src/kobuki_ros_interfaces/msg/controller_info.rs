use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ControllerInfo {
    pub r#type: u8,
    pub p_gain: f64,
    pub i_gain: f64,
    pub d_gain: f64,
}

impl ControllerInfo {
    pub const DEFAULT: u8 = 0;
    pub const USER_CONFIGURED: u8 = 1;
}

impl Default for ControllerInfo {
    fn default() -> Self {
        ControllerInfo {
            r#type: 0,
            p_gain: 0.0,
            i_gain: 0.0,
            d_gain: 0.0,
        }
    }
}
