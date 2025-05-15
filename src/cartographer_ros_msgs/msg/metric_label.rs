use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct MetricLabel {
    pub key: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for MetricLabel {
    fn default() -> Self {
        MetricLabel {
            key: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}
