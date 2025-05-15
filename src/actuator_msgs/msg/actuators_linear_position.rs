use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ActuatorsLinearPosition {
    pub position: Vec<f64>,
}

impl Default for ActuatorsLinearPosition {
    fn default() -> Self {
        ActuatorsLinearPosition {
            position: Vec::new(),
        }
    }
}
