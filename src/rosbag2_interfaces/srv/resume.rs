use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ResumeRequest {}

impl Default for ResumeRequest {
    fn default() -> Self {
        ResumeRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ResumeResponse {}

impl Default for ResumeResponse {
    fn default() -> Self {
        ResumeResponse {}
    }
}

pub struct Resume;
