use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointStiffnesses {
    pub indexes: Vec<u8>,
    pub stiffnesses: Vec<f32>,
}

impl Default for JointStiffnesses {
    fn default() -> Self {
        JointStiffnesses {
            indexes: Vec::new(),
            stiffnesses: Vec::new(),
        }
    }
}
