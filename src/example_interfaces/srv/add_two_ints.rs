use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddTwoIntsRequest {
    pub a: i64,
    pub b: i64,
}

impl Default for AddTwoIntsRequest {
    fn default() -> Self {
        AddTwoIntsRequest { a: 0, b: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddTwoIntsResponse {
    pub sum: i64,
}

impl Default for AddTwoIntsResponse {
    fn default() -> Self {
        AddTwoIntsResponse { sum: 0 }
    }
}

pub struct AddTwoInts;
