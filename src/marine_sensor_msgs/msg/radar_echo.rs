use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RadarEcho {
    pub echoes: Vec<f32>,
}

impl Default for RadarEcho {
    fn default() -> Self {
        RadarEcho { echoes: Vec::new() }
    }
}
