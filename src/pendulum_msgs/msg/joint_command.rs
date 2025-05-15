use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JointCommand {
    pub position: f64,
}

impl Default for JointCommand {
    fn default() -> Self {
        JointCommand { position: 0.0 }
    }
}
