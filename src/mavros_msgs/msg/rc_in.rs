use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RCIn {
    pub header: crate::std_msgs::msg::Header,
    pub rssi: u8,
    pub channels: Vec<u16>,
}

impl Default for RCIn {
    fn default() -> Self {
        RCIn {
            header: crate::std_msgs::msg::Header::default(),
            rssi: 0,
            channels: Vec::new(),
        }
    }
}
