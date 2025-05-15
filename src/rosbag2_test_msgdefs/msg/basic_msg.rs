use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BasicMsg {
    pub c: f32,
}

impl Default for BasicMsg {
    fn default() -> Self {
        BasicMsg { c: 0.0 }
    }
}
