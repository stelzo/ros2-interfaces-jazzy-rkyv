use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CameraDiagnostics {
    pub header: crate::std_msgs::msg::Header,
    pub data: Vec<crate::diagnostic_msgs::msg::KeyValue>,
}

impl Default for CameraDiagnostics {
    fn default() -> Self {
        CameraDiagnostics {
            header: crate::std_msgs::msg::Header::default(),
            data: Vec::new(),
        }
    }
}
