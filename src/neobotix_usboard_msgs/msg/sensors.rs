use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Sensors {
    pub header: crate::std_msgs::msg::Header,
    pub sensors: Vec<crate::neobotix_usboard_msgs::msg::SensorData>,
}

impl Default for Sensors {
    fn default() -> Self {
        Sensors {
            header: crate::std_msgs::msg::Header::default(),
            sensors: Vec::new(),
        }
    }
}
