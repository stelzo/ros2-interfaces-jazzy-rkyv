use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PublishMapRequest {
    pub global_map: bool,
    pub optimized: bool,
    pub graph_only: bool,
}

impl Default for PublishMapRequest {
    fn default() -> Self {
        PublishMapRequest {
            global_map: false,
            optimized: false,
            graph_only: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct PublishMapResponse {}

impl Default for PublishMapResponse {
    fn default() -> Self {
        PublishMapResponse {}
    }
}

pub struct PublishMap;
