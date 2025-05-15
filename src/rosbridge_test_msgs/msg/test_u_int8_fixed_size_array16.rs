use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestUInt8FixedSizeArray16 {
    pub data: [u8; 16],
}

impl Default for TestUInt8FixedSizeArray16 {
    fn default() -> Self {
        TestUInt8FixedSizeArray16 { data: [0; 16] }
    }
}
