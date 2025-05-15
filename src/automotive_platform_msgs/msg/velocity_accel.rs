use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct VelocityAccel {
    pub header: crate::std_msgs::msg::Header,
    pub velocity: f32,
    pub accleration: f32,
}

impl Default for VelocityAccel {
    fn default() -> Self {
        VelocityAccel {
            header: crate::std_msgs::msg::Header::default(),
            velocity: 0.0,
            accleration: 0.0,
        }
    }
}
