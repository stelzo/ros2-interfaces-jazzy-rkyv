use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SelfTestRequest {}

impl Default for SelfTestRequest {
    fn default() -> Self {
        SelfTestRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SelfTestResponse {
    pub id: ::std::string::String,
    pub passed: u8,
    pub status: Vec<crate::diagnostic_msgs::msg::DiagnosticStatus>,
}

impl Default for SelfTestResponse {
    fn default() -> Self {
        SelfTestResponse {
            id: ::std::string::String::new(),
            passed: 0,
            status: Vec::new(),
        }
    }
}

pub struct SelfTest;
