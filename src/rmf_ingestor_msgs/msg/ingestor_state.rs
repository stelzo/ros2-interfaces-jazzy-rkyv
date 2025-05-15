use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IngestorState {
    pub time: crate::builtin_interfaces::msg::Time,
    pub guid: ::std::string::String,
    pub mode: i32,
    pub request_guid_queue: Vec<::std::string::String>,
    pub seconds_remaining: f32,
}

impl IngestorState {
    pub const IDLE: i32 = 0;
    pub const BUSY: i32 = 1;
    pub const OFFLINE: i32 = 2;
}

impl Default for IngestorState {
    fn default() -> Self {
        IngestorState {
            time: crate::builtin_interfaces::msg::Time::default(),
            guid: ::std::string::String::new(),
            mode: 0,
            request_guid_queue: Vec::new(),
            seconds_remaining: 0.0,
        }
    }
}
