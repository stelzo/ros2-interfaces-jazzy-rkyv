use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FloatingPointRange {
    pub from_value: f64,
    pub to_value: f64,
    pub step: f64,
}

impl Default for FloatingPointRange {
    fn default() -> Self {
        FloatingPointRange {
            from_value: 0.0,
            to_value: 0.0,
            step: 0.0,
        }
    }
}
