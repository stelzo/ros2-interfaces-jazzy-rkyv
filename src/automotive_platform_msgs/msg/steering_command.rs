use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SteeringCommand {
    pub header: crate::std_msgs::msg::Header,
    pub steering_wheel_angle: f32,
}

impl Default for SteeringCommand {
    fn default() -> Self {
        SteeringCommand {
            header: crate::std_msgs::msg::Header::default(),
            steering_wheel_angle: 0.0,
        }
    }
}
