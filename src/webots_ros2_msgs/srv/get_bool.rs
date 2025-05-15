use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetBoolRequest {
    pub ask: bool,
}

impl Default for GetBoolRequest {
    fn default() -> Self {
        GetBoolRequest { ask: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetBoolResponse {
    pub value: bool,
}

impl Default for GetBoolResponse {
    fn default() -> Self {
        GetBoolResponse { value: false }
    }
}

pub struct GetBool;
