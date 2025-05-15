use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IngestorResult {
    pub time: crate::builtin_interfaces::msg::Time,
    pub request_guid: ::std::string::String,
    pub source_guid: ::std::string::String,
    pub status: u8,
}

impl IngestorResult {
    pub const ACKNOWLEDGED: u8 = 0;
    pub const SUCCESS: u8 = 1;
    pub const FAILED: u8 = 2;
}

impl Default for IngestorResult {
    fn default() -> Self {
        IngestorResult {
            time: crate::builtin_interfaces::msg::Time::default(),
            request_guid: ::std::string::String::new(),
            source_guid: ::std::string::String::new(),
            status: 0,
        }
    }
}
