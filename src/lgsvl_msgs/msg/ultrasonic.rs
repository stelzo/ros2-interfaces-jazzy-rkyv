use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Ultrasonic {
    pub header: crate::std_msgs::msg::Header,
    pub minimum_distance: f32,
}

impl Default for Ultrasonic {
    fn default() -> Self {
        Ultrasonic {
            header: crate::std_msgs::msg::Header::default(),
            minimum_distance: 0.0,
        }
    }
}
