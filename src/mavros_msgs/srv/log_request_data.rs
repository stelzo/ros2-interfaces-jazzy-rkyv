use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LogRequestDataRequest {
    pub id: u16,
    pub offset: u32,
    pub count: u32,
}

impl Default for LogRequestDataRequest {
    fn default() -> Self {
        LogRequestDataRequest {
            id: 0,
            offset: 0,
            count: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LogRequestDataResponse {
    pub success: bool,
}

impl Default for LogRequestDataResponse {
    fn default() -> Self {
        LogRequestDataResponse { success: false }
    }
}

pub struct LogRequestData;
