use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct SmachContainerInitialStatusCmd {
    pub path: ::std::string::String,
    pub initial_states: Vec<::std::string::String>,
    pub local_data: Vec<u8>,
}

impl Default for SmachContainerInitialStatusCmd {
    fn default() -> Self {
        SmachContainerInitialStatusCmd {
            path: ::std::string::String::new(),
            initial_states: Vec::new(),
            local_data: Vec::new(),
        }
    }
}
