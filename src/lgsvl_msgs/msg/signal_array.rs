use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SignalArray {
    pub header: crate::std_msgs::msg::Header,
    pub signals: Vec<crate::lgsvl_msgs::msg::Signal>,
}

impl Default for SignalArray {
    fn default() -> Self {
        SignalArray {
            header: crate::std_msgs::msg::Header::default(),
            signals: Vec::new(),
        }
    }
}
