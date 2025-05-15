use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListLabelsRequest {}

impl Default for ListLabelsRequest {
    fn default() -> Self {
        ListLabelsRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListLabelsResponse {
    pub ids: Vec<i32>,
    pub labels: Vec<::std::string::String>,
}

impl Default for ListLabelsResponse {
    fn default() -> Self {
        ListLabelsResponse {
            ids: Vec::new(),
            labels: Vec::new(),
        }
    }
}

pub struct ListLabels;
