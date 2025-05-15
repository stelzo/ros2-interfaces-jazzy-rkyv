use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct ReadSplitEvent {
    pub closed_file: ::std::string::String,
    pub opened_file: ::std::string::String,
    pub node_name: ::std::string::String,
}

impl Default for ReadSplitEvent {
    fn default() -> Self {
        ReadSplitEvent {
            closed_file: ::std::string::String::new(),
            opened_file: ::std::string::String::new(),
            node_name: ::std::string::String::new(),
        }
    }
}
