use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLoadedProgramRequest {}

impl Default for GetLoadedProgramRequest {
    fn default() -> Self {
        GetLoadedProgramRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetLoadedProgramResponse {
    pub answer: ::std::string::String,
    pub program_name: ::std::string::String,
    pub success: bool,
}

impl Default for GetLoadedProgramResponse {
    fn default() -> Self {
        GetLoadedProgramResponse {
            answer: ::std::string::String::new(),
            program_name: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct GetLoadedProgram;
