use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetContaminationResultSrvRequest {}

impl Default for GetContaminationResultSrvRequest {
    fn default() -> Self {
        GetContaminationResultSrvRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetContaminationResultSrvResponse {
    pub warning: u8,
    pub error: u8,
    pub success: bool,
}

impl Default for GetContaminationResultSrvResponse {
    fn default() -> Self {
        GetContaminationResultSrvResponse {
            warning: 0,
            error: 0,
            success: false,
        }
    }
}

pub struct GetContaminationResultSrv;
