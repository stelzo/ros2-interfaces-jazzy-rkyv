use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetImuCalibrationRequest {
    pub gyro_bias_x: f64,
    pub gyro_bias_y: f64,
    pub gyro_bias_z: f64,
}

impl Default for SetImuCalibrationRequest {
    fn default() -> Self {
        SetImuCalibrationRequest {
            gyro_bias_x: 0.0,
            gyro_bias_y: 0.0,
            gyro_bias_z: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetImuCalibrationResponse {
    pub success: bool,
}

impl Default for SetImuCalibrationResponse {
    fn default() -> Self {
        SetImuCalibrationResponse { success: false }
    }
}

pub struct SetImuCalibration;
