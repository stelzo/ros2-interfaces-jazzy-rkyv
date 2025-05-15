use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IrIntensity {
    pub header: crate::std_msgs::msg::Header,
    pub value: i16,
}

impl Default for IrIntensity {
    fn default() -> Self {
        IrIntensity {
            header: crate::std_msgs::msg::Header::default(),
            value: 0,
        }
    }
}
