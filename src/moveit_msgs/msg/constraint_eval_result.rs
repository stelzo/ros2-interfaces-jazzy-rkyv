use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ConstraintEvalResult {
    pub result: bool,
    pub distance: f64,
}

impl Default for ConstraintEvalResult {
    fn default() -> Self {
        ConstraintEvalResult {
            result: false,
            distance: 0.0,
        }
    }
}
