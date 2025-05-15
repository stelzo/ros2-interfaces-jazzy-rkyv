use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Actuators {
    pub header: crate::std_msgs::msg::Header,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub normalized: Vec<f64>,
}

impl Default for Actuators {
    fn default() -> Self {
        Actuators {
            header: crate::std_msgs::msg::Header::default(),
            position: Vec::new(),
            velocity: Vec::new(),
            normalized: Vec::new(),
        }
    }
}
