use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestUInt8 {
    pub data: Vec<u8>,
}

impl Default for TestUInt8 {
    fn default() -> Self {
        TestUInt8 { data: Vec::new() }
    }
}
