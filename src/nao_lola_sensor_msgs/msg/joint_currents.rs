use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointCurrents {
    pub currents: [f32; 25],
}

impl Default for JointCurrents {
    fn default() -> Self {
        JointCurrents {
            currents: [0.0; 25],
        }
    }
}
