use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct EncoderDecimatedSpeed {
    pub header: crate::std_msgs::msg::Header,
    pub avr_speed: f64,
}

impl Default for EncoderDecimatedSpeed {
    fn default() -> Self {
        EncoderDecimatedSpeed {
            header: crate::std_msgs::msg::Header::default(),
            avr_speed: 0.0,
        }
    }
}
