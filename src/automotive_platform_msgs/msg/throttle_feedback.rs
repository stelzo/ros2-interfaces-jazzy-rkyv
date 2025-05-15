use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ThrottleFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub throttle_pedal: f32,
}

impl Default for ThrottleFeedback {
    fn default() -> Self {
        ThrottleFeedback {
            header: crate::std_msgs::msg::Header::default(),
            throttle_pedal: 0.0,
        }
    }
}
