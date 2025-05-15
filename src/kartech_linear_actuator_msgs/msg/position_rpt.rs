use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PositionRpt {
    pub header: crate::std_msgs::msg::Header,
    pub shaft_extension: f64,
}

impl Default for PositionRpt {
    fn default() -> Self {
        PositionRpt {
            header: crate::std_msgs::msg::Header::default(),
            shaft_extension: 0.0,
        }
    }
}
