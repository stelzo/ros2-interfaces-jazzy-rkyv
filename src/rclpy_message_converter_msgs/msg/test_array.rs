use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestArray {
    pub data: Vec<f64>,
}

impl Default for TestArray {
    fn default() -> Self {
        TestArray { data: Vec::new() }
    }
}
