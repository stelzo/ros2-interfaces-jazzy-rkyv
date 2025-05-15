use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SerializedTransaction {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<u8>,
}

impl Default for SerializedTransaction {
    fn default() -> Self {
        SerializedTransaction {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}
