use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Altitude {
    pub header: crate::std_msgs::msg::Header,
    pub altitude: f64,
    pub sigma: f64,
}

impl Default for Altitude {
    fn default() -> Self {
        Altitude {
            header: crate::std_msgs::msg::Header::default(),
            altitude: 0.0,
            sigma: 0.0,
        }
    }
}
