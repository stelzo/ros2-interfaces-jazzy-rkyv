use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RemoveLabelRequest {
    pub label: ::std::string::String,
}

impl Default for RemoveLabelRequest {
    fn default() -> Self {
        RemoveLabelRequest {
            label: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct RemoveLabelResponse {}

impl Default for RemoveLabelResponse {
    fn default() -> Self {
        RemoveLabelResponse {}
    }
}

pub struct RemoveLabel;
