use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct CommandFeedback {
    pub command: ::std::string::String,
    pub args: Vec<::std::string::String>,
}

impl Default for CommandFeedback {
    fn default() -> Self {
        CommandFeedback {
            command: ::std::string::String::new(),
            args: Vec::new(),
        }
    }
}
