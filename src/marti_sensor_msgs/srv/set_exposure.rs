use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetExposureRequest {
    pub auto_exposure: bool,
    pub time: i64,
}

impl Default for SetExposureRequest {
    fn default() -> Self {
        SetExposureRequest {
            auto_exposure: false,
            time: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetExposureResponse {
    pub auto_exposure: bool,
    pub time: i64,
}

impl Default for SetExposureResponse {
    fn default() -> Self {
        SetExposureResponse {
            auto_exposure: false,
            time: 0,
        }
    }
}

pub struct SetExposure;
