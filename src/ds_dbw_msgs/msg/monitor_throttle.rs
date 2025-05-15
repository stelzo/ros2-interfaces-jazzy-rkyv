use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MonitorThrottle {
    pub header: crate::std_msgs::msg::Header,
    pub pedal_pc: f32,
    pub pedal_qf: crate::ds_dbw_msgs::msg::Quality,
}

impl Default for MonitorThrottle {
    fn default() -> Self {
        MonitorThrottle {
            header: crate::std_msgs::msg::Header::default(),
            pedal_pc: 0.0,
            pedal_qf: crate::ds_dbw_msgs::msg::Quality::default(),
        }
    }
}
