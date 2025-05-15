use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct HandLandmarkArray {
    pub header: crate::std_msgs::msg::Header,
    pub landmarks: Vec<crate::depthai_ros_msgs::msg::HandLandmark>,
}

impl Default for HandLandmarkArray {
    fn default() -> Self {
        HandLandmarkArray {
            header: crate::std_msgs::msg::Header::default(),
            landmarks: Vec::new(),
        }
    }
}
