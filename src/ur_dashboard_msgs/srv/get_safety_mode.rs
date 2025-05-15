use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetSafetyModeRequest {}

impl Default for GetSafetyModeRequest {
    fn default() -> Self {
        GetSafetyModeRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetSafetyModeResponse {
    pub safety_mode: crate::ur_dashboard_msgs::msg::SafetyMode,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetSafetyModeResponse {
    fn default() -> Self {
        GetSafetyModeResponse {
            safety_mode: crate::ur_dashboard_msgs::msg::SafetyMode::default(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct GetSafetyMode;
