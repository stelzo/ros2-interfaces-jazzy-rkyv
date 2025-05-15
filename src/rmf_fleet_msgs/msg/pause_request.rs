use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PauseRequest {
    pub fleet_name: ::std::string::String,
    pub robot_name: ::std::string::String,
    pub mode_request_id: u64,
    pub r#type: u32,
    pub at_checkpoint: u32,
}

impl PauseRequest {
    pub const TYPE_PAUSE_IMMEDIATELY: u32 = 0;
    pub const TYPE_PAUSE_AT_CHECKPOINT: u32 = 1;
    pub const TYPE_RESUME: u32 = 2;
}

impl Default for PauseRequest {
    fn default() -> Self {
        PauseRequest {
            fleet_name: ::std::string::String::new(),
            robot_name: ::std::string::String::new(),
            mode_request_id: 0,
            r#type: 0,
            at_checkpoint: 0,
        }
    }
}
