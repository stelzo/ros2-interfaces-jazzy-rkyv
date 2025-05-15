use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MultiEchoLaserScan {
    pub header: crate::std_msgs::msg::Header,
    pub angle_min: f32,
    pub angle_max: f32,
    pub angle_increment: f32,
    pub time_increment: f32,
    pub scan_time: f32,
    pub range_min: f32,
    pub range_max: f32,
    pub ranges: Vec<crate::sensor_msgs::msg::LaserEcho>,
    pub intensities: Vec<crate::sensor_msgs::msg::LaserEcho>,
}

impl Default for MultiEchoLaserScan {
    fn default() -> Self {
        MultiEchoLaserScan {
            header: crate::std_msgs::msg::Header::default(),
            angle_min: 0.0,
            angle_max: 0.0,
            angle_increment: 0.0,
            time_increment: 0.0,
            scan_time: 0.0,
            range_min: 0.0,
            range_max: 0.0,
            ranges: Vec::new(),
            intensities: Vec::new(),
        }
    }
}
