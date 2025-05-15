use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CreateScheduleRequest {
    pub schedule: crate::rmf_scheduler_msgs::msg::Schedule,
}

impl Default for CreateScheduleRequest {
    fn default() -> Self {
        CreateScheduleRequest {
            schedule: crate::rmf_scheduler_msgs::msg::Schedule::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CreateScheduleResponse {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CreateScheduleResponse {
    fn default() -> Self {
        CreateScheduleResponse {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

pub struct CreateSchedule;
