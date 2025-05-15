use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RobotPowerRequest {}

impl Default for RobotPowerRequest {
    fn default() -> Self {
        RobotPowerRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RobotPowerResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for RobotPowerResponse {
    fn default() -> Self {
        RobotPowerResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct RobotPower;
