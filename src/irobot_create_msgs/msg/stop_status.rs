use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StopStatus {
    pub header: crate::std_msgs::msg::Header,
    pub is_stopped: bool,
}

impl Default for StopStatus {
    fn default() -> Self {
        StopStatus {
            header: crate::std_msgs::msg::Header::default(),
            is_stopped: false,
        }
    }
}
