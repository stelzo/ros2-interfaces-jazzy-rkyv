use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListScheduleStatesRequest {
    pub modified_after: i64,
}

impl Default for ListScheduleStatesRequest {
    fn default() -> Self {
        ListScheduleStatesRequest { modified_after: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListScheduleStatesResponse {
    pub success: bool,
    pub message: ::std::string::String,
    pub schedules: Vec<crate::rmf_scheduler_msgs::msg::ScheduleState>,
}

impl Default for ListScheduleStatesResponse {
    fn default() -> Self {
        ListScheduleStatesResponse {
            success: false,
            message: ::std::string::String::new(),
            schedules: Vec::new(),
        }
    }
}

pub struct ListScheduleStates;
