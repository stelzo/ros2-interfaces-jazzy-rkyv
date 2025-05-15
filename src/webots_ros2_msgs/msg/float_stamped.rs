use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FloatStamped {
    pub header: crate::std_msgs::msg::Header,
    pub data: f64,
}

impl Default for FloatStamped {
    fn default() -> Self {
        FloatStamped {
            header: crate::std_msgs::msg::Header::default(),
            data: 0.0,
        }
    }
}
