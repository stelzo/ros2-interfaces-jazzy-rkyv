use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AmbientLight {
    pub value: u8,
}

impl AmbientLight {
    pub const UNKNOWN: u8 = 0;
    pub const DARK: u8 = 1;
    pub const MEDIUM: u8 = 2;
    pub const LIGHT: u8 = 3;
}

impl Default for AmbientLight {
    fn default() -> Self {
        AmbientLight { value: 0 }
    }
}
