use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FrameGraphRequest {}

impl Default for FrameGraphRequest {
    fn default() -> Self {
        FrameGraphRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct FrameGraphResponse {
    pub frame_yaml: ::std::string::String,
}

impl Default for FrameGraphResponse {
    fn default() -> Self {
        FrameGraphResponse {
            frame_yaml: ::std::string::String::new(),
        }
    }
}

pub struct FrameGraph;
