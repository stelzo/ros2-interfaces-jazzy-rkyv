use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SrrDebug5 {
    pub header: crate::std_msgs::msg::Header,
    pub can_tx_align_updates: u16,
}

impl Default for SrrDebug5 {
    fn default() -> Self {
        SrrDebug5 {
            header: crate::std_msgs::msg::Header::default(),
            can_tx_align_updates: 0,
        }
    }
}
