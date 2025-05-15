use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct JoyFeedbackArray {
    pub array: Vec<crate::sensor_msgs::msg::JoyFeedback>,
}

impl Default for JoyFeedbackArray {
    fn default() -> Self {
        JoyFeedbackArray { array: Vec::new() }
    }
}
