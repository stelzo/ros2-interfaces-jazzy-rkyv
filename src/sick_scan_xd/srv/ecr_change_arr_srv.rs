use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ECRChangeArrSrvRequest {
    pub active: bool,
}

impl Default for ECRChangeArrSrvRequest {
    fn default() -> Self {
        ECRChangeArrSrvRequest { active: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ECRChangeArrSrvResponse {
    pub success: bool,
}

impl Default for ECRChangeArrSrvResponse {
    fn default() -> Self {
        ECRChangeArrSrvResponse { success: false }
    }
}

pub struct ECRChangeArrSrv;
