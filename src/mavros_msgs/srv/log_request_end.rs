use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LogRequestEndRequest {}

impl Default for LogRequestEndRequest {
    fn default() -> Self {
        LogRequestEndRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LogRequestEndResponse {
    pub success: bool,
}

impl Default for LogRequestEndResponse {
    fn default() -> Self {
        LogRequestEndResponse { success: false }
    }
}

pub struct LogRequestEnd;
