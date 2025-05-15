use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Battery {
    pub charge: f32,
    pub charging: bool,
    pub current: f32,
    pub temperature: f32,
}

impl Default for Battery {
    fn default() -> Self {
        Battery {
            charge: 0.0,
            charging: false,
            current: 0.0,
            temperature: 0.0,
        }
    }
}
