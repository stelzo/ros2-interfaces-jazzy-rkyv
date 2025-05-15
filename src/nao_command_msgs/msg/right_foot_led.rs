use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RightFootLed {
    pub color: crate::std_msgs::msg::ColorRGBA,
}

impl Default for RightFootLed {
    fn default() -> Self {
        RightFootLed {
            color: crate::std_msgs::msg::ColorRGBA::default(),
        }
    }
}
