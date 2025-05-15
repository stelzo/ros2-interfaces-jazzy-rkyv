use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ListParametersResult {
    pub names: Vec<::std::string::String>,
    pub prefixes: Vec<::std::string::String>,
}

impl Default for ListParametersResult {
    fn default() -> Self {
        ListParametersResult {
            names: Vec::new(),
            prefixes: Vec::new(),
        }
    }
}
