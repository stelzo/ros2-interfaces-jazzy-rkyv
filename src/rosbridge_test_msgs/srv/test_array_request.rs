use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestArrayRequestRequest {
    pub int_values: Vec<i32>,
}

impl Default for TestArrayRequestRequest {
    fn default() -> Self {
        TestArrayRequestRequest {
            int_values: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestArrayRequestResponse {}

impl Default for TestArrayRequestResponse {
    fn default() -> Self {
        TestArrayRequestResponse {}
    }
}

pub struct TestArrayRequest;
