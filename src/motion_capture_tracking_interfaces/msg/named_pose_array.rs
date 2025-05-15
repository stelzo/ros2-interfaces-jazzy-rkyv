use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct NamedPoseArray {
    pub header: crate::std_msgs::msg::Header,
    pub poses: Vec<crate::motion_capture_tracking_interfaces::msg::NamedPose>,
}

impl Default for NamedPoseArray {
    fn default() -> Self {
        NamedPoseArray {
            header: crate::std_msgs::msg::Header::default(),
            poses: Vec::new(),
        }
    }
}
