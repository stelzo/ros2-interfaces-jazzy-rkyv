use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SickScanExitSrvRequest {}

impl Default for SickScanExitSrvRequest {
    fn default() -> Self {
        SickScanExitSrvRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SickScanExitSrvResponse {
    pub success: bool,
}

impl Default for SickScanExitSrvResponse {
    fn default() -> Self {
        SickScanExitSrvResponse { success: false }
    }
}

pub struct SickScanExitSrv;
