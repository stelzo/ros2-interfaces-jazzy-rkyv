use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestRequestOnlyRequest {
    pub data: i32,
}

impl Default for TestRequestOnlyRequest {
    fn default() -> Self {
        TestRequestOnlyRequest { data: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestRequestOnlyResponse {}

impl Default for TestRequestOnlyResponse {
    fn default() -> Self {
        TestRequestOnlyResponse {}
    }
}

pub struct TestRequestOnly;
