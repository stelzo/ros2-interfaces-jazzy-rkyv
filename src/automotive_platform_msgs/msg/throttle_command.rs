use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ThrottleCommand {
    pub header: crate::std_msgs::msg::Header,
    pub throttle_pedal: f32,
}

impl Default for ThrottleCommand {
    fn default() -> Self {
        ThrottleCommand {
            header: crate::std_msgs::msg::Header::default(),
            throttle_pedal: 0.0,
        }
    }
}
