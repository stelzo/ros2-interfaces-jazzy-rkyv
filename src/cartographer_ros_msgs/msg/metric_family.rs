use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MetricFamily {
    pub name: ::std::string::String,
    pub description: ::std::string::String,
    pub metrics: Vec<crate::cartographer_ros_msgs::msg::Metric>,
}

impl Default for MetricFamily {
    fn default() -> Self {
        MetricFamily {
            name: ::std::string::String::new(),
            description: ::std::string::String::new(),
            metrics: Vec::new(),
        }
    }
}
