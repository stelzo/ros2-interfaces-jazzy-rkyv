use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ScanAngle {
    pub header: crate::std_msgs::msg::Header,
    pub scan_angle: f64,
}

impl Default for ScanAngle {
    fn default() -> Self {
        ScanAngle {
            header: crate::std_msgs::msg::Header::default(),
            scan_angle: 0.0,
        }
    }
}
