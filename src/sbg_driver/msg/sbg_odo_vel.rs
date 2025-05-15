use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SbgOdoVel {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub status: bool,
    pub vel: f32,
}

impl Default for SbgOdoVel {
    fn default() -> Self {
        SbgOdoVel {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            status: false,
            vel: 0.0,
        }
    }
}
