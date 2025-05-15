use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TopicsAndRawTypesRequest {}

impl Default for TopicsAndRawTypesRequest {
    fn default() -> Self {
        TopicsAndRawTypesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct TopicsAndRawTypesResponse {
    pub topics: Vec<::std::string::String>,
    pub types: Vec<::std::string::String>,
    pub typedefs_full_text: Vec<::std::string::String>,
}

impl Default for TopicsAndRawTypesResponse {
    fn default() -> Self {
        TopicsAndRawTypesResponse {
            topics: Vec::new(),
            types: Vec::new(),
            typedefs_full_text: Vec::new(),
        }
    }
}

pub struct TopicsAndRawTypes;
