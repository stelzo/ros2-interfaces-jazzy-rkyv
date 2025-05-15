use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ModelCoefficients {
    pub header: crate::std_msgs::msg::Header,
    pub values: Vec<f32>,
}

impl Default for ModelCoefficients {
    fn default() -> Self {
        ModelCoefficients {
            header: crate::std_msgs::msg::Header::default(),
            values: Vec::new(),
        }
    }
}
