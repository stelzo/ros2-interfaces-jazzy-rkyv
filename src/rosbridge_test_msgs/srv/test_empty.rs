use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestEmptyRequest {}

impl Default for TestEmptyRequest {
    fn default() -> Self {
        TestEmptyRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestEmptyResponse {}

impl Default for TestEmptyResponse {
    fn default() -> Self {
        TestEmptyResponse {}
    }
}

pub struct TestEmpty;
