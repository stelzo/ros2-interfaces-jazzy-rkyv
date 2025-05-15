use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EmptyRequest {}

impl Default for EmptyRequest {
    fn default() -> Self {
        EmptyRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EmptyResponse {}

impl Default for EmptyResponse {
    fn default() -> Self {
        EmptyResponse {}
    }
}

pub struct Empty;
