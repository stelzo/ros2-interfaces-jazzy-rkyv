use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLoggerLevelsRequest {
    pub levels: Vec<crate::rcl_interfaces::msg::LoggerLevel>,
}

impl Default for SetLoggerLevelsRequest {
    fn default() -> Self {
        SetLoggerLevelsRequest { levels: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLoggerLevelsResponse {
    pub results: Vec<crate::rcl_interfaces::msg::SetLoggerLevelsResult>,
}

impl Default for SetLoggerLevelsResponse {
    fn default() -> Self {
        SetLoggerLevelsResponse {
            results: Vec::new(),
        }
    }
}

pub struct SetLoggerLevels;
