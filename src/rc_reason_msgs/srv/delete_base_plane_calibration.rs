use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteBasePlaneCalibrationRequest {}

impl Default for DeleteBasePlaneCalibrationRequest {
    fn default() -> Self {
        DeleteBasePlaneCalibrationRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DeleteBasePlaneCalibrationResponse {
    pub return_code: crate::rc_common_msgs::msg::ReturnCode,
}

impl Default for DeleteBasePlaneCalibrationResponse {
    fn default() -> Self {
        DeleteBasePlaneCalibrationResponse {
            return_code: crate::rc_common_msgs::msg::ReturnCode::default(),
        }
    }
}

pub struct DeleteBasePlaneCalibration;
