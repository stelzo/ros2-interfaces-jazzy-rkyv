use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestFloat32Array {
    pub data: Vec<f32>,
}

impl Default for TestFloat32Array {
    fn default() -> Self {
        TestFloat32Array { data: Vec::new() }
    }
}
