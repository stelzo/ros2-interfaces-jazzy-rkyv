use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListControllerTypesRequest {}

impl Default for ListControllerTypesRequest {
    fn default() -> Self {
        ListControllerTypesRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListControllerTypesResponse {
    pub types: Vec<::std::string::String>,
    pub base_classes: Vec<::std::string::String>,
}

impl Default for ListControllerTypesResponse {
    fn default() -> Self {
        ListControllerTypesResponse {
            types: Vec::new(),
            base_classes: Vec::new(),
        }
    }
}

pub struct ListControllerTypes;
