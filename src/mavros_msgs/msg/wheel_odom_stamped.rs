use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WheelOdomStamped {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<f64>,
}

impl Default for WheelOdomStamped {
    fn default() -> Self {
        WheelOdomStamped {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}
