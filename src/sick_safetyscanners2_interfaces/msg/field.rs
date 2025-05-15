use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Field {
    pub ranges: Vec<f32>,
    pub start_angle: f32,
    pub angular_resolution: f32,
    pub protective_field: bool,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            ranges: Vec::new(),
            start_angle: 0.0,
            angular_resolution: 0.0,
            protective_field: false,
        }
    }
}
