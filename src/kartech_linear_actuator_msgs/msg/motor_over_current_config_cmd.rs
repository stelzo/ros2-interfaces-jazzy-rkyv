use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MotorOverCurrentConfigCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub over_current: u16,
}

impl Default for MotorOverCurrentConfigCmd {
    fn default() -> Self {
        MotorOverCurrentConfigCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            over_current: 0,
        }
    }
}
