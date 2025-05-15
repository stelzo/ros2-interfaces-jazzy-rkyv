use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProgramStateRequest {}

impl Default for GetProgramStateRequest {
    fn default() -> Self {
        GetProgramStateRequest {}
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct GetProgramStateResponse {
    pub state: crate::ur_dashboard_msgs::msg::ProgramState,
    pub program_name: ::std::string::String,
    pub answer: ::std::string::String,
    pub success: bool,
}

impl Default for GetProgramStateResponse {
    fn default() -> Self {
        GetProgramStateResponse {
            state: crate::ur_dashboard_msgs::msg::ProgramState::default(),
            program_name: ::std::string::String::new(),
            answer: ::std::string::String::new(),
            success: false,
        }
    }
}

pub struct GetProgramState;
