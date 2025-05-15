use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ActuatorsNormalized {
    pub header: crate::std_msgs::msg::Header,
    pub normalized: Vec<f64>,
}

impl Default for ActuatorsNormalized {
    fn default() -> Self {
        ActuatorsNormalized {
            header: crate::std_msgs::msg::Header::default(),
            normalized: Vec::new(),
        }
    }
}
