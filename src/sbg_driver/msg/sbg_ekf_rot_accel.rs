use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SbgEkfRotAccel {
    pub header: crate::std_msgs::msg::Header,
    pub time_stamp: u32,
    pub rate: crate::geometry_msgs::msg::Vector3,
    pub acceleration: crate::geometry_msgs::msg::Vector3,
}

impl Default for SbgEkfRotAccel {
    fn default() -> Self {
        SbgEkfRotAccel {
            header: crate::std_msgs::msg::Header::default(),
            time_stamp: 0,
            rate: crate::geometry_msgs::msg::Vector3::default(),
            acceleration: crate::geometry_msgs::msg::Vector3::default(),
        }
    }
}
