use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReadMetricsRequest {}

impl Default for ReadMetricsRequest {
    fn default() -> Self {
        ReadMetricsRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReadMetricsResponse {
    pub status: crate::cartographer_ros_msgs::msg::StatusResponse,
    pub metric_families: Vec<crate::cartographer_ros_msgs::msg::MetricFamily>,
    pub timestamp: crate::builtin_interfaces::msg::Time,
}

impl Default for ReadMetricsResponse {
    fn default() -> Self {
        ReadMetricsResponse {
            status: crate::cartographer_ros_msgs::msg::StatusResponse::default(),
            metric_families: Vec::new(),
            timestamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

pub struct ReadMetrics;
