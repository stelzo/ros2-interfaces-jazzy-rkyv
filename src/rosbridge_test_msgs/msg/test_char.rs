use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestChar {
    pub data: Vec<i8>,
}

impl Default for TestChar {
    fn default() -> Self {
        TestChar { data: Vec::new() }
    }
}
