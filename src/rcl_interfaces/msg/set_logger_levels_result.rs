use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SetLoggerLevelsResult {
    pub successful: bool,
    pub reason: ::std::string::String,
}

impl Default for SetLoggerLevelsResult {
    fn default() -> Self {
        SetLoggerLevelsResult {
            successful: false,
            reason: ::std::string::String::new(),
        }
    }
}
