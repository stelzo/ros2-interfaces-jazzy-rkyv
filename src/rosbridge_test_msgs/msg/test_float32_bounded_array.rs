use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestFloat32BoundedArray {
    pub data: [f32; 16],
}

impl Default for TestFloat32BoundedArray {
    fn default() -> Self {
        TestFloat32BoundedArray { data: [0.0; 16] }
    }
}
