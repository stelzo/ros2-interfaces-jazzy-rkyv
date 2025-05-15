use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct State {
    pub state: u8,
    pub node_name: ::std::string::String,
}

impl Default for State {
    fn default() -> Self {
        State {
            state: 0,
            node_name: ::std::string::String::new(),
        }
    }
}
