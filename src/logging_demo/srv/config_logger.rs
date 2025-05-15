use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ConfigLoggerRequest {
    pub logger_name: ::std::string::String,
    pub level: ::std::string::String,
}

impl Default for ConfigLoggerRequest {
    fn default() -> Self {
        ConfigLoggerRequest {
            logger_name: ::std::string::String::new(),
            level: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ConfigLoggerResponse {
    pub success: bool,
}

impl Default for ConfigLoggerResponse {
    fn default() -> Self {
        ConfigLoggerResponse { success: false }
    }
}

pub struct ConfigLogger;
