use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ESCTelemetry {
    pub header: crate::std_msgs::msg::Header,
    pub esc_telemetry: Vec<crate::mavros_msgs::msg::ESCTelemetryItem>,
}

impl Default for ESCTelemetry {
    fn default() -> Self {
        ESCTelemetry {
            header: crate::std_msgs::msg::Header::default(),
            esc_telemetry: Vec::new(),
        }
    }
}
