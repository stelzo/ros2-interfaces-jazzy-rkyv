use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Float64Stamped {
    pub header: crate::std_msgs::msg::Header,
    pub value: f64,
}

impl Default for Float64Stamped {
    fn default() -> Self {
        Float64Stamped {
            header: crate::std_msgs::msg::Header::default(),
            value: 0.0,
        }
    }
}
