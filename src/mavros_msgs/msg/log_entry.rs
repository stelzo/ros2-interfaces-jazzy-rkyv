use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LogEntry {
    pub header: crate::std_msgs::msg::Header,
    pub id: u16,
    pub num_logs: u16,
    pub last_log_num: u16,
    pub time_utc: crate::builtin_interfaces::msg::Time,
    pub size: u32,
}

impl Default for LogEntry {
    fn default() -> Self {
        LogEntry {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            num_logs: 0,
            last_log_num: 0,
            time_utc: crate::builtin_interfaces::msg::Time::default(),
            size: 0,
        }
    }
}
