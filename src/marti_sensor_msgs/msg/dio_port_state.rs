use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct DioPortState {
    pub header: crate::std_msgs::msg::Header,
    pub value: u64,
}

impl Default for DioPortState {
    fn default() -> Self {
        DioPortState {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
