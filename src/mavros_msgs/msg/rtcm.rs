use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RTCM {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<u8>,
}

impl Default for RTCM {
    fn default() -> Self {
        RTCM {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}
