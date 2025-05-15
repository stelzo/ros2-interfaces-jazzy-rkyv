use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddProblemRequest {
    pub problem: ::std::string::String,
}

impl Default for AddProblemRequest {
    fn default() -> Self {
        AddProblemRequest {
            problem: ::std::string::String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct AddProblemResponse {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AddProblemResponse {
    fn default() -> Self {
        AddProblemResponse {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

pub struct AddProblem;
