use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetParametersResult {
    pub successful: bool,
    pub reason: ::std::string::String,
}

impl Default for SetParametersResult {
    fn default() -> Self {
        SetParametersResult {
            successful: false,
            reason: ::std::string::String::new(),
        }
    }
}
