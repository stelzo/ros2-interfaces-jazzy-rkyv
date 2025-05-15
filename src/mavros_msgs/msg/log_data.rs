use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct LogData {
    pub header: crate::std_msgs::msg::Header,
    pub id: u16,
    pub offset: u32,
    pub data: Vec<u8>,
}

impl Default for LogData {
    fn default() -> Self {
        LogData {
            header: crate::std_msgs::msg::Header::default(),
            id: 0,
            offset: 0,
            data: Vec::new(),
        }
    }
}
