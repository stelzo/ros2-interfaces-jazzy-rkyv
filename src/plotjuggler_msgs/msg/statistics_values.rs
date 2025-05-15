use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StatisticsValues {
    pub header: crate::std_msgs::msg::Header,
    pub values: Vec<f64>,
    pub names_version: u32,
}

impl Default for StatisticsValues {
    fn default() -> Self {
        StatisticsValues {
            header: crate::std_msgs::msg::Header::default(),
            values: Vec::new(),
            names_version: 0,
        }
    }
}
