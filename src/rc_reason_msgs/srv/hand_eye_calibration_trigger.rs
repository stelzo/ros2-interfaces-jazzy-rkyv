use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HandEyeCalibrationTriggerRequest {}

impl Default for HandEyeCalibrationTriggerRequest {
    fn default() -> Self {
        HandEyeCalibrationTriggerRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HandEyeCalibrationTriggerResponse {
    pub success: bool,
    pub status: i32,
    pub message: ::std::string::String,
}

impl Default for HandEyeCalibrationTriggerResponse {
    fn default() -> Self {
        HandEyeCalibrationTriggerResponse {
            success: false,
            status: 0,
            message: ::std::string::String::new(),
        }
    }
}

pub struct HandEyeCalibrationTrigger;
