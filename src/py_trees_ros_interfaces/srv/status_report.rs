use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StatusReportRequest {}

impl Default for StatusReportRequest {
    fn default() -> Self {
        StatusReportRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct StatusReportResponse {
    pub report: ::std::string::String,
}

impl Default for StatusReportResponse {
    fn default() -> Self {
        StatusReportResponse {
            report: ::std::string::String::new(),
        }
    }
}

pub struct StatusReport;
