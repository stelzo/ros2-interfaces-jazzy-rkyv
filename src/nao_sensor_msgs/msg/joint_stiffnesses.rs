use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointStiffnesses {
    pub stiffnesses: [f32; 25],
}

impl Default for JointStiffnesses {
    fn default() -> Self {
        JointStiffnesses {
            stiffnesses: [0.0; 25],
        }
    }
}
