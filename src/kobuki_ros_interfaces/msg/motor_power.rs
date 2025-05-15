use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MotorPower {
    pub state: u8,
}

impl MotorPower {
    pub const OFF: u8 = 0;
    pub const ON: u8 = 1;
}

impl Default for MotorPower {
    fn default() -> Self {
        MotorPower { state: 0 }
    }
}
