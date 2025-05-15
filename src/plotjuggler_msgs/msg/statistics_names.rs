use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StatisticsNames {
    pub header: crate::std_msgs::msg::Header,
    pub names: Vec<::std::string::String>,
    pub names_version: u32,
}

impl Default for StatisticsNames {
    fn default() -> Self {
        StatisticsNames {
            header: crate::std_msgs::msg::Header::default(),
            names: Vec::new(),
            names_version: 0,
        }
    }
}
