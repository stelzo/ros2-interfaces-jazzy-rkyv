use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsProgramSavedRequest {}

impl Default for IsProgramSavedRequest {
    fn default() -> Self {
        IsProgramSavedRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct IsProgramSavedResponse {
    pub answer: ::std::string::String,
    pub program_name: ::std::string::String,
    pub program_saved: bool,
    pub success: bool,
}

impl Default for IsProgramSavedResponse {
    fn default() -> Self {
        IsProgramSavedResponse {
            answer: ::std::string::String::new(),
            program_name: ::std::string::String::new(),
            program_saved: false,
            success: false,
        }
    }
}

pub struct IsProgramSaved;
