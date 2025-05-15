use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct WheelTicks {
    pub header: crate::std_msgs::msg::Header,
    pub ticks_left: i32,
    pub ticks_right: i32,
}

impl Default for WheelTicks {
    fn default() -> Self {
        WheelTicks {
            header: crate::std_msgs::msg::Header::default(),
            ticks_left: 0,
            ticks_right: 0,
        }
    }
}
