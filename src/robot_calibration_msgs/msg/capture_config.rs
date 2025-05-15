use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CaptureConfig {
    pub joint_states: crate::sensor_msgs::msg::JointState,
    pub features: Vec<::std::string::String>,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        CaptureConfig {
            joint_states: crate::sensor_msgs::msg::JointState::default(),
            features: Vec::new(),
        }
    }
}
