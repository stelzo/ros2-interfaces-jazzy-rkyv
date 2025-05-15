use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LaserEcho {
    pub echoes: Vec<f32>,
}

impl Default for LaserEcho {
    fn default() -> Self {
        LaserEcho { echoes: Vec::new() }
    }
}
