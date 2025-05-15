use rkyv::{Archive, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Archive)]
pub struct State {
    pub header: crate::std_msgs::msg::Header,
    pub state_name: ::std::string::String,
    pub state: u8,
}

impl Default for State {
    fn default() -> Self {
        State {
            header: crate::std_msgs::msg::Header::default(),
            state_name: ::std::string::String::new(),
            state: 0,
        }
    }
}
