use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ResetODORequest {}

impl Default for ResetODORequest {
    fn default() -> Self {
        ResetODORequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ResetODOResponse {}

impl Default for ResetODOResponse {
    fn default() -> Self {
        ResetODOResponse {}
    }
}

pub struct ResetODO;
