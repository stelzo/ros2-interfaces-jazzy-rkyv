use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RadarScan {
    pub header: crate::std_msgs::msg::Header,
    pub returns: Vec<crate::radar_msgs::msg::RadarReturn>,
}

impl Default for RadarScan {
    fn default() -> Self {
        RadarScan {
            header: crate::std_msgs::msg::Header::default(),
            returns: Vec::new(),
        }
    }
}
