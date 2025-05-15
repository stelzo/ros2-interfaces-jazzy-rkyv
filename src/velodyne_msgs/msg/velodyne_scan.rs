use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct VelodyneScan {
    pub header: crate::std_msgs::msg::Header,
    pub packets: Vec<crate::velodyne_msgs::msg::VelodynePacket>,
}

impl Default for VelodyneScan {
    fn default() -> Self {
        VelodyneScan {
            header: crate::std_msgs::msg::Header::default(),
            packets: Vec::new(),
        }
    }
}
