use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ActuatorsAngularVelocity {
    pub velocity: Vec<f64>,
}

impl Default for ActuatorsAngularVelocity {
    fn default() -> Self {
        ActuatorsAngularVelocity {
            velocity: Vec::new(),
        }
    }
}
