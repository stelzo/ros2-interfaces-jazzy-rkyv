use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Velocity {
    pub header: crate::std_msgs::msg::Header,
    pub velocity: f64,
    pub variance: f64,
}

impl Default for Velocity {
    fn default() -> Self {
        Velocity {
            header: crate::std_msgs::msg::Header::default(),
            velocity: 0.0,
            variance: 0.0,
        }
    }
}
