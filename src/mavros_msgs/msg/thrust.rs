use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Thrust {
    pub header: crate::std_msgs::msg::Header,
    pub thrust: f32,
}

impl Default for Thrust {
    fn default() -> Self {
        Thrust {
            header: crate::std_msgs::msg::Header::default(),
            thrust: 0.0,
        }
    }
}
