use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointPositions {
    pub indexes: Vec<u8>,
    pub positions: Vec<f32>,
}

impl Default for JointPositions {
    fn default() -> Self {
        JointPositions {
            indexes: Vec::new(),
            positions: Vec::new(),
        }
    }
}
