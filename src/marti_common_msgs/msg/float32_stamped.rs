use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Float32Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: f32,
}

impl Default for Float32Stamped {
    fn default() -> Self {
        Float32Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0.0,
        }
    }
}
