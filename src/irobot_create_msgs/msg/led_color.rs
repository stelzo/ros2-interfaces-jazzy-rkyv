use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LedColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Default for LedColor {
    fn default() -> Self {
        LedColor {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}
