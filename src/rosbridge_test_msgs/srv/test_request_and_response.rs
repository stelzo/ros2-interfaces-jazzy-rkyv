use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestRequestAndResponseRequest {
    pub data: i32,
}

impl Default for TestRequestAndResponseRequest {
    fn default() -> Self {
        TestRequestAndResponseRequest { data: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TestRequestAndResponseResponse {
    pub data: i32,
}

impl Default for TestRequestAndResponseResponse {
    fn default() -> Self {
        TestRequestAndResponseResponse { data: 0 }
    }
}

pub struct TestRequestAndResponse;
