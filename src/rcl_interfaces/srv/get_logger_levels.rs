use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLoggerLevelsRequest {
    pub names: Vec<::std::string::String>,
}

impl Default for GetLoggerLevelsRequest {
    fn default() -> Self {
        GetLoggerLevelsRequest { names: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLoggerLevelsResponse {
    pub levels: Vec<crate::rcl_interfaces::msg::LoggerLevel>,
}

impl Default for GetLoggerLevelsResponse {
    fn default() -> Self {
        GetLoggerLevelsResponse { levels: Vec::new() }
    }
}

pub struct GetLoggerLevels;
