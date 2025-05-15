use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ActuatorsLinearVelocity {
    pub velocity: Vec<f64>,
}

impl Default for ActuatorsLinearVelocity {
    fn default() -> Self {
        ActuatorsLinearVelocity {
            velocity: Vec::new(),
        }
    }
}
