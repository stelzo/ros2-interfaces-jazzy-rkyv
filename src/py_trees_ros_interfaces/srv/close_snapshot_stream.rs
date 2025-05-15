use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CloseSnapshotStreamRequest {
    pub topic_name: ::std::string::String,
}

impl Default for CloseSnapshotStreamRequest {
    fn default() -> Self {
        CloseSnapshotStreamRequest {
            topic_name: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CloseSnapshotStreamResponse {
    pub result: bool,
}

impl Default for CloseSnapshotStreamResponse {
    fn default() -> Self {
        CloseSnapshotStreamResponse { result: false }
    }
}

pub struct CloseSnapshotStream;
