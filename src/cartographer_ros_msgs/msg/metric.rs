use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct Metric {
    pub r#type: u8,
    pub labels: Vec<crate::cartographer_ros_msgs::msg::MetricLabel>,
    pub value: f64,
    pub counts_by_bucket: Vec<crate::cartographer_ros_msgs::msg::HistogramBucket>,
}

impl Metric {
    pub const TYPE_COUNTER: u8 = 0;
    pub const TYPE_GAUGE: u8 = 1;
    pub const TYPE_HISTOGRAM: u8 = 2;
}

impl Default for Metric {
    fn default() -> Self {
        Metric {
            r#type: 0,
            labels: Vec::new(),
            value: 0.0,
            counts_by_bucket: Vec::new(),
        }
    }
}
