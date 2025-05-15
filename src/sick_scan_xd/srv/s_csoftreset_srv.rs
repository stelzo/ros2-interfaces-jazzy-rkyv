use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SCsoftresetSrvRequest {}

impl Default for SCsoftresetSrvRequest {
    fn default() -> Self {
        SCsoftresetSrvRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SCsoftresetSrvResponse {
    pub success: bool,
}

impl Default for SCsoftresetSrvResponse {
    fn default() -> Self {
        SCsoftresetSrvResponse { success: false }
    }
}

pub struct SCsoftresetSrv;
