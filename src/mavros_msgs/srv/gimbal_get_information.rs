use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GimbalGetInformationRequest {}

impl Default for GimbalGetInformationRequest {
    fn default() -> Self {
        GimbalGetInformationRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GimbalGetInformationResponse {
    pub success: bool,
    pub result: u8,
}

impl Default for GimbalGetInformationResponse {
    fn default() -> Self {
        GimbalGetInformationResponse {
            success: false,
            result: 0,
        }
    }
}

pub struct GimbalGetInformation;
