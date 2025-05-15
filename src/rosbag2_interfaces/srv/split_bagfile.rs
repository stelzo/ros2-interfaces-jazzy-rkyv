use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SplitBagfileRequest {}

impl Default for SplitBagfileRequest {
    fn default() -> Self {
        SplitBagfileRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SplitBagfileResponse {}

impl Default for SplitBagfileResponse {
    fn default() -> Self {
        SplitBagfileResponse {}
    }
}

pub struct SplitBagfile;
