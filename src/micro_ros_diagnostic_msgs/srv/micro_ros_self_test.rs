use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MicroROSSelfTestRequest {}

impl Default for MicroROSSelfTestRequest {
    fn default() -> Self {
        MicroROSSelfTestRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MicroROSSelfTestResponse {
    pub id: ::std::string::String,
    pub passed: u8,
    pub status: crate::micro_ros_diagnostic_msgs::msg::MicroROSDiagnosticStatus,
}

impl Default for MicroROSSelfTestResponse {
    fn default() -> Self {
        MicroROSSelfTestResponse {
            id: ::std::string::String::new(),
            passed: 0,
            status: crate::micro_ros_diagnostic_msgs::msg::MicroROSDiagnosticStatus::default(),
        }
    }
}

pub struct MicroROSSelfTest;
