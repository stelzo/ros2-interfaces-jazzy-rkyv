use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CancelScheduleRequest {
    pub name: ::std::string::String,
    pub finished: bool,
}

impl Default for CancelScheduleRequest {
    fn default() -> Self {
        CancelScheduleRequest {
            name: ::std::string::String::new(),
            finished: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CancelScheduleResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelScheduleResponse {
    fn default() -> Self {
        CancelScheduleResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct CancelSchedule;
