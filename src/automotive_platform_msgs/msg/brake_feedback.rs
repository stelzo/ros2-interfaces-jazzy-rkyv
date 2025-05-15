use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BrakeFeedback {
    pub header: crate::std_msgs::msg::Header,
    pub brake_pedal: f32,
}

impl Default for BrakeFeedback {
    fn default() -> Self {
        BrakeFeedback {
            header: crate::std_msgs::msg::Header::default(),
            brake_pedal: 0.0,
        }
    }
}
