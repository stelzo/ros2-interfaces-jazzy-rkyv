use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct BrakeCommand {
    pub header: crate::std_msgs::msg::Header,
    pub brake_pedal: f32,
}

impl Default for BrakeCommand {
    fn default() -> Self {
        BrakeCommand {
            header: crate::std_msgs::msg::Header::default(),
            brake_pedal: 0.0,
        }
    }
}
