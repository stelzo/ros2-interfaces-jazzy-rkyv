use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsProgramRunningRequest {}

impl Default for IsProgramRunningRequest {
    fn default() -> Self {
        IsProgramRunningRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsProgramRunningResponse {
    pub answer: ::std::string::String,
    pub program_running: bool,
    pub success: bool,
}

impl Default for IsProgramRunningResponse {
    fn default() -> Self {
        IsProgramRunningResponse {
            answer: ::std::string::String::new(),
            program_running: false,
            success: false,
        }
    }
}

pub struct IsProgramRunning;
