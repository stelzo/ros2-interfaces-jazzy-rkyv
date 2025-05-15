use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLabelRequest {
    pub node_id: i32,
    pub node_label: ::std::string::String,
}

impl Default for SetLabelRequest {
    fn default() -> Self {
        SetLabelRequest {
            node_id: 0,
            node_label: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLabelResponse {}

impl Default for SetLabelResponse {
    fn default() -> Self {
        SetLabelResponse {}
    }
}

pub struct SetLabel;
