use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TFMessage {
    pub transforms: Vec<crate::geometry_msgs::msg::TransformStamped>,
}

impl Default for TFMessage {
    fn default() -> Self {
        TFMessage {
            transforms: Vec::new(),
        }
    }
}
