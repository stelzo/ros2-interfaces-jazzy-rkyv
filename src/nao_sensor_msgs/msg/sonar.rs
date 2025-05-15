use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Sonar {
    pub left: f32,
    pub right: f32,
}

impl Default for Sonar {
    fn default() -> Self {
        Sonar {
            left: 0.0,
            right: 0.0,
        }
    }
}
