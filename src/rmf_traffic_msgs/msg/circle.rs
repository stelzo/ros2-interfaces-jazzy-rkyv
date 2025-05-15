use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Circle {
    pub radius: f64,
}

impl Default for Circle {
    fn default() -> Self {
        Circle { radius: 0.0 }
    }
}
