use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SCdevicestateSrvRequest {}

impl Default for SCdevicestateSrvRequest {
    fn default() -> Self {
        SCdevicestateSrvRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SCdevicestateSrvResponse {
    pub state: i32,
    pub success: bool,
}

impl Default for SCdevicestateSrvResponse {
    fn default() -> Self {
        SCdevicestateSrvResponse {
            state: 0,
            success: false,
        }
    }
}

pub struct SCdevicestateSrv;
