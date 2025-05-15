use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointTemperatures {
    pub temperatures: [f32; 25],
}

impl Default for JointTemperatures {
    fn default() -> Self {
        JointTemperatures {
            temperatures: [0.0; 25],
        }
    }
}
