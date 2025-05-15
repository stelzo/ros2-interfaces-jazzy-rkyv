use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CameraControl {
    pub header: crate::std_msgs::msg::Header,
    pub exposure_time: u32,
    pub gain: f32,
}

impl Default for CameraControl {
    fn default() -> Self {
        CameraControl {
            header: crate::std_msgs::msg::Header::default(),
            exposure_time: 0,
            gain: 0.0,
        }
    }
}
