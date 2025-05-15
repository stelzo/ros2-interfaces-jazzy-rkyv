use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PositionReachErrorTimeConfigCmd {
    pub header: crate::std_msgs::msg::Header,
    pub confirm: bool,
    pub position_reach_error_time: u16,
}

impl Default for PositionReachErrorTimeConfigCmd {
    fn default() -> Self {
        PositionReachErrorTimeConfigCmd {
            header: crate::std_msgs::msg::Header::default(),
            confirm: false,
            position_reach_error_time: 0,
        }
    }
}
