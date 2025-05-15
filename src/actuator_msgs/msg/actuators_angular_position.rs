use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ActuatorsAngularPosition {
    pub position: Vec<f64>,
}

impl Default for ActuatorsAngularPosition {
    fn default() -> Self {
        ActuatorsAngularPosition {
            position: Vec::new(),
        }
    }
}
