use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ChestLed {
    pub color: crate::std_msgs::msg::ColorRGBA,
}

impl Default for ChestLed {
    fn default() -> Self {
        ChestLed {
            color: crate::std_msgs::msg::ColorRGBA::default(),
        }
    }
}
