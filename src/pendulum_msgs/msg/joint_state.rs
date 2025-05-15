use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointState {
    pub position: f64,
    pub velocity: f64,
    pub effort: f64,
}

impl Default for JointState {
    fn default() -> Self {
        JointState {
            position: 0.0,
            velocity: 0.0,
            effort: 0.0,
        }
    }
}
