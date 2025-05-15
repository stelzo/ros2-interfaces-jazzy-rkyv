use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CameraRecognitionObjects {
    pub header: crate::std_msgs::msg::Header,
    pub objects: Vec<crate::webots_ros2_msgs::msg::CameraRecognitionObject>,
}

impl Default for CameraRecognitionObjects {
    fn default() -> Self {
        CameraRecognitionObjects {
            header: crate::std_msgs::msg::Header::default(),
            objects: Vec::new(),
        }
    }
}
