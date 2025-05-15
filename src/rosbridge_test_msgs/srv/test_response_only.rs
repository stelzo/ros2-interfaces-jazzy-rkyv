use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestResponseOnlyRequest {}

impl Default for TestResponseOnlyRequest {
    fn default() -> Self {
        TestResponseOnlyRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestResponseOnlyResponse {
    pub data: i32,
}

impl Default for TestResponseOnlyResponse {
    fn default() -> Self {
        TestResponseOnlyResponse { data: 0 }
    }
}

pub struct TestResponseOnly;
